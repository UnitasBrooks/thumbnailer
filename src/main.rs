use ffmpeg_next as ffmpeg;

use ffmpeg::{
    codec,
    format,
    frame,
    media,
    software::scaling::{context::Context, flag::Flags},
};

use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ffmpeg::init()?;

    // Example RTP TS input
    let input_url = "rtp://127.0.0.1:5004";

    let mut ictx = format::input(&input_url)?;

    // Find best video stream
    let input_stream = ictx
        .streams()
        .best(media::Type::Video)
        .ok_or("No video stream")?;

    let video_stream_index = input_stream.index();

    // Create decoder
    let context_decoder =
        codec::context::Context::from_parameters(input_stream.parameters())?;

    let mut decoder = context_decoder.decoder().video()?;

    // Scaling context (to RGB)
    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg::format::Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;

    let mut decoded = frame::Video::empty();
    let mut rgb_frame = frame::Video::empty();

    println!("Waiting for first keyframe...");

    for (stream, packet) in ictx.packets() {
        if stream.index() != video_stream_index {
            continue;
        }

        decoder.send_packet(&packet)?;

        while decoder.receive_frame(&mut decoded).is_ok() {

            if decoded.is_key() {
                println!("Found keyframe!");

                scaler.run(&decoded, &mut rgb_frame)?;

                save_as_jpeg(&rgb_frame, "first_frame.jpg")?;

                println!("Saved first_frame.jpg");

                return Ok(());
            }
        }
    }

    Ok(())
}

fn save_as_jpeg(
    frame: &frame::Video,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {

    use ffmpeg::codec::Id;
    use ffmpeg::encoder;

    let codec = encoder::find(Id::MJPEG).ok_or("MJPEG codec not found")?;

    let mut context = codec::context::Context::new();
    let mut encoder = context.encoder().video()?;

    encoder.set_width(frame.width());
    encoder.set_height(frame.height());
    encoder.set_format(ffmpeg::format::Pixel::YUVJ420P);

    let mut encoder = encoder.open_as(codec)?;

    encoder.send_frame(frame)?;

    let mut packet = ffmpeg::Packet::empty();
    encoder.receive_packet(&mut packet)?;

    let mut file = File::create(path)?;
    file.write_all(packet.data().unwrap())?;

    Ok(())
}