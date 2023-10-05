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
        let mut image = buffer.decode_image::<LumaFormat>().unwrap();
        visr::equalize_mut(&mut image);
        let points: Vec<_> = fastr::fast(&image)
            .into_iter()
            .map(|kp| rerun::components::Point2D::new(kp.0 as f32, kp.1 as f32))
            .collect();
        let t = rerun::components::Tensor::from_image(image).expect("msg");
        rerun::MsgSender::new("image/equalized")
            .with_component(&[t])
            .expect("")
            .send(&recording)
            .expect("");
        rerun::MsgSender::new("image/keypoints")
            .with_component(&points)
            .expect("")
            .send(&recording)
            .expect("");
    })?;
    threaded.open_stream()?;
    loop {
        let frame = threaded.poll_frame()?;
        let image = frame.decode_image::<LumaFormat>()?;
        println!("frame polled: {}x{}", image.width(), image.height());
    }
}
