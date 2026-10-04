fn main() {}

mod encoding {
    use anyhow::Context;
    use muxide::api::{MuxerBuilder, VideoCodec};
    use openh264::{
        OpenH264API,
        encoder::{BitRate, Encoder, EncoderConfig, FrameRate},
        formats::{RgbaSliceU8, YUVBuffer},
    };
    use std::fs::File;

    // TODO create a config struct
    pub fn try_encode(
        //
        frames: impl IntoIterator<Item = Vec<u8>>,
    ) -> anyhow::Result<()> {
        let width = 1920;
        let height = 1080;
        let fps = 60;
        let duration_secs = 10;

        let config = EncoderConfig::new()
            .bitrate(BitRate::from_bps(500_000))
            .max_frame_rate(FrameRate::from_hz(1. / fps as f32));

        let mut enc = Encoder::with_api_config(OpenH264API::from_source(), config)?;

        let file = File::create("out.mp4")?;
        let mut mux = MuxerBuilder::new(file)
            .video(VideoCodec::H264, width, height, fps as _)
            .build()?;

        let mut frames = frames.into_iter();

        for _ in 0..(fps * duration_secs) {
            let frame_data = frames.next().with_context(|| "No more frames")?;

            let rgb_source = RgbaSliceU8::new(&frame_data, (width as _, height as _));
            let yuv_source = YUVBuffer::from_rgb_source(rgb_source);

            let bitstream = enc.encode(&yuv_source)?;
            mux.encode_video(&bitstream.to_vec(), 1000 / fps)?;
        }

        dbg!(mux.finish_with_stats()?);

        Ok(())
    }
}

mod framegen {
    // Call typst-render::render(page, pixel_per_pt) -> sk::pixmap ~= Vec<u8> RGBA
    pub fn gen_frame() -> anyhow::Result<Vec<u8>> {
        todo!()
    }
}

// cli
// elean-arrow video.typ --fps 30
// MAYBE its interesting to be able to define gen options from typst, and override them from cli
//
// - parse
// - loop t
//  - eval + layout
//  - render
//  - encode + mux
//
// MAYBE make an iterator that generates frames of the animation
