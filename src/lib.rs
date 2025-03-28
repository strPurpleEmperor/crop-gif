use image::codecs::gif::{GifDecoder, GifEncoder, Repeat};
use image::imageops::crop;
use image::{AnimationDecoder, Delay, Frame, RgbaImage};
use js_sys::Uint8Array;
use std::io::Cursor;
use std::vec::Vec;
use wasm_bindgen::prelude::*;
// For better panic messages in browser console
#[cfg(feature = "console_error_panic_hook")]
pub use console_error_panic_hook::set_once as set_panic_hook;

#[wasm_bindgen]
pub fn crop_gif(gif_data: &[u8], w: u32, h: u32, x: u32, y: u32) -> Result<Uint8Array, JsValue> {
    let cursor = Cursor::new(gif_data);
    // 解码 GIF
    let decoder = GifDecoder::new(cursor).map_err(|_| JsValue::from_str("GIF 解码失败"))?;
    let frames = decoder
        .into_frames()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| JsValue::from_str("无法收集 GIF 帧"))?;

    let mut mirrored_frames: Vec<(RgbaImage, Delay)> = vec![];
    for frame in frames {
        let delay = frame.delay();
        let mut image = frame.into_buffer();
        let new_image = crop_image(&mut image, w, h, x, y);
        mirrored_frames.push((new_image, delay));
    }

    // 编码为新的 GIF
    let mut output_data = Vec::new();
    {
        let mut encoder = GifEncoder::new(&mut output_data);
        encoder
            .set_repeat(Repeat::Infinite)
            .map_err(|_| JsValue::from_str("无法设置 GIF 循环"))?;
        for (frame, delay) in mirrored_frames {
            let gif_frame = Frame::from_parts(frame, 0, 0, delay);
            encoder
                .encode_frame(gif_frame)
                .map_err(|_| JsValue::from_str("无法编码 GIF 帧"))?;
        }
    }

    Ok(Uint8Array::from(&output_data[..]))
}

fn crop_image(image: &mut RgbaImage, w: u32, h: u32, x: u32, y: u32) -> RgbaImage {
    let cropped = crop(image, x, y, w, h);
    cropped.to_image()
}