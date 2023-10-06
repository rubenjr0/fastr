use eyre::Result;
use nokhwa::{
    pixel_format::LumaFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType, Resolution},
    CallbackCamera,
};

fn main() -> Result<()> {
    let recording = rerun::RecordingStreamBuilder::new("FASTR")
        .connect(rerun::default_server_addr(), rerun::default_flush_timeout())?;
    let camera_idx = CameraIndex::Index(0);
    let format = RequestedFormat::new::<LumaFormat>(RequestedFormatType::HighestResolution(
        Resolution::new(1280, 720),
    ));

    let mut threaded = CallbackCamera::new(camera_idx, format, move |buffer| {
        let image = buffer.decode_image::<LumaFormat>().unwrap();
        let points = fastr::fast(&image);
        let points = rerun::Points2D::new(points.into_iter().map(|(x, y)| (x as f32, y as f32)));
        recording
            .log("luma", &rerun::Image::try_from(image).unwrap())
            .unwrap();
        recording.log("luma/keypoints", &points).unwrap();
    })?;
    threaded.open_stream()?;
    loop {
        let frame = threaded.poll_frame()?;
        let image = frame.decode_image::<LumaFormat>()?;
        println!("frame polled: {}x{}", image.width(), image.height());
    }
}
