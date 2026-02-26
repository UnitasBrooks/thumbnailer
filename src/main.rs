use ffmpeg_next::{self as ffmpeg, Dictionary};

use ffmpeg::{
    codec,
    format,
    frame,
    media,
};

use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ffmpeg::init()?;

    let input_url = "rtp://127.0.0.1:5004";

    let mut opts = Dictionary::new();
    opts.set("fflags", "+genpts");
    opts.set("analyzeduration", "5000000");
    opts.set("probesize", "5000000");

    let mut ictx = format::input_with_dictionary(
        &input_url,
        opts
    )?;

    let input_stream = ictx
        .streams()
        .best(media::Type::Video)
        .ok_or("No video stream")?;

    let video_stream_index = input_stream.index();

    let context_decoder =
        codec::context::Context::from_parameters(input_stream.parameters())?;

    let mut decoder = context_decoder.decoder().video()?;
    let mut decoded = frame::Video::empty();


    println!("Waiting for first keyframe...");

    for (stream, packet) in ictx.packets() {
        if stream.index() != video_stream_index {
            continue;
        }

        decoder.send_packet(&packet)?;

        while decoder.receive_frame(&mut decoded).is_ok() {

            if decoded.is_key() {
                println!("Found keyframe!");
                save_as_jpeg(&decoded, "frame.raw")?;
                println!("Saved frame.raw");
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

    let mut file = File::create(path)?;
    println!("format: {:?}", frame.format());
    println!("planes: {}", frame.planes());
    println!("width: {}", frame.width());
    println!("height: {}", frame.height());
    println!("stride_y: {}", frame.stride(0));
    println!("stride_u: {}", frame.stride(1));
    println!("stride_v: {}", frame.stride(2));
    println!("data_y len: {}", frame.data(0).len());
    println!("data_u len: {}", frame.data(1).len());
    println!("data_v len: {}", frame.data(2).len());
    let width = frame.width() as usize;
    let height = frame.height() as usize;

    let stride_y = frame.stride(0) as usize;
    let data_y = frame.data(0);

    let mut height_div = 2;

    if frame.format() == format::Pixel::YUV422P {
        height_div = 1
    }

    for y in 0..height {
        let row = &data_y[y * stride_y .. y * stride_y + width];
        file.write_all(row)?;
    }

    let stride_u = frame.stride(1) as usize;
    let data_u = frame.data(1);

    for y in 0..height/height_div {
        let row = &data_u[y * stride_u .. y * stride_u + width/2];
        file.write_all(row)?;
    }

    let stride_v = frame.stride(2) as usize;
    let data_v = frame.data(2);

    for y in 0..height/height_div {
        let row = &data_v[y * stride_v .. y * stride_v + width/2];
        file.write_all(row)?;
    }

    Ok(())
}