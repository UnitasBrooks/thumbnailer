# YUV422P
ffmpeg -re -f lavfi -i "testsrc=size=1280x720:rate=30" -f lavfi -f rtp_mpegts -c:v mpeg2video -pix_fmt yuv422p rtp://127.0.0.1:5004
ffmpeg -f rawvideo -pixel_format yuv422p -video_size 1280x720 -i frame.raw output_yuv422.jpg


# YUV420P
ffmpeg -re -f lavfi -i "testsrc=size=1280x720:rate=30" -f lavfi -f rtp_mpegts rtp://127.0.0.1:5004
ffmpeg -f rawvideo -pixel_format yuv420p -video_size 1280x720 -i frame.raw output_yuv420.jpg

