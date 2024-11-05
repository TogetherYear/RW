#![allow(non_snake_case)]
use base64::{engine::general_purpose, Engine};
use image::DynamicImage;
use std::io::{Cursor, Read, Seek, SeekFrom};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn ConvertPngImageFileToBase64(buf: &[u8]) -> Result<String, String> {
    match GetImageFromBuffer(&buf) {
        Ok(img) => match GetBase64FromImage(&img) {
            Ok(b) => Ok(b),
            Err(_) => Err(format!("转换base64失败")),
        },
        Err(_) => Err(format!("请传入png图片")),
    }
}

pub fn GetImageFromBuffer(buf: &[u8]) -> Result<DynamicImage, JsValue> {
    match image::load_from_memory_with_format(buf, image::ImageFormat::Png) {
        Ok(img) => Ok(img),
        Err(_) => Err(JsValue::null()),
    }
}

pub fn GetBase64FromImage(img: &DynamicImage) -> Result<String, JsValue> {
    let mut c = Cursor::new(Vec::new());
    match img.write_to(&mut c, image::ImageFormat::Png) {
        Ok(_) => {
            c.seek(SeekFrom::Start(0)).unwrap();
            let mut out = Vec::new();
            c.read_to_end(&mut out).unwrap();
            let b = general_purpose::STANDARD.encode(&mut out);
            Ok(format!("{}{}", "data:image/png;base64,", b))
        }
        Err(_) => Err(JsValue::null()),
    }
}
