use std::{slice, fs::File, io::Read, ffi::{c_char, CStr, c_int}, sync::Mutex, collections::HashMap, borrow::BorrowMut};
use anyhow::{Result, anyhow, Ok};
use gif::{Decoder, Encoder, Repeat, Frame};
use log::info;
use once_cell::sync::Lazy;
use raqote::{DrawTarget, Image, DrawOptions};

/// 返回的是argb数据
pub type DecodeCallback = extern fn(*const u8, i32, i32);

pub struct EncoderInfo{
    encoder: Encoder<File>,
    fps: i32,
    _repeat: bool,
    width: i32,
    height: i32,
}

pub static ENCODERS: Lazy<Mutex<HashMap<String, EncoderInfo>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub fn create_encoder(file: *const c_char, _repeat: bool, width: c_int, height: c_int, fps:c_int) -> Result<i32>{
    let file_name = unsafe{ CStr::from_ptr(file) };
    let image = File::create(file_name.to_str()?)?;
    let mut encoder = Encoder::new(image, width as u16, height as u16, &[])?;
    if _repeat{
        encoder.set_repeat(Repeat::Infinite)?;
    }
    info!("创建GIF编码器: {} repeat={_repeat} {width}x{height} FPS:{fps}", file_name.to_str()?);
    ENCODERS.lock().map_err(|err| anyhow!("{:?}", err))?
    .borrow_mut()
    .insert(file_name.to_str()?.to_string(), EncoderInfo{
        encoder,
        fps,
        _repeat,
        width,
        height
    });
    Ok(0)
}

pub fn append_file_to_encoder(file: *const c_char, image_file: *const c_char) -> Result<i32>{
    let image_file_name = unsafe{ CStr::from_ptr(image_file).to_str()?.to_string() };
    let mut image = image::open(image_file_name)?.to_rgba8();
    append_rgba_to_decoder(file, &mut image)
}

pub fn append_file_data_to_encoder(file: *const c_char, image_file: *const u8, len: i32) -> Result<i32>{
    let file_data = unsafe{ slice::from_raw_parts(image_file, len as usize) };
    let image = image::load_from_memory(file_data)?;
    let mut image = image.to_rgba8();
    append_rgba_to_decoder(file, &mut image)
}

pub fn close_decoder(file: *const c_char) -> Result<i32>{
    let encoder_file_name = unsafe{ CStr::from_ptr(file).to_str()?.to_string() };
    let mut encoders = ENCODERS.lock().map_err(|err| anyhow!("{:?}", err))?;
    let encoders = encoders.borrow_mut();

    //drop
    match encoders.remove(&encoder_file_name){
        Some(encoder_info) => {
            info!("encoder已关闭:{:?} {}x{}", encoder_file_name, encoder_info.width, encoder_info.height);
            Ok(0)
        }
        None => Err(anyhow!("Eecoder不存在!"))
    }
}

fn append_rgba_to_decoder<'a>(file: *const c_char, pixel_data:&'a mut [u8] ) -> Result<i32>{
    let encoder_file_name = unsafe{ CStr::from_ptr(file).to_str()?.to_string() };
    let mut encoders = ENCODERS.lock().map_err(|err| anyhow!("{:?}", err))?;
    let encoders = encoders.borrow_mut();
    match encoders.get_mut(&encoder_file_name){
        Some(encoder_info) => {
            let mut frame = Frame::from_rgba(encoder_info.width as u16, encoder_info.height as u16, pixel_data);
            frame.delay = 1000 / encoder_info.fps as u16 / 10; //设置帧率 10ms倍数
            info!("delay 帧延迟:{} encoder_info.fps={}", frame.delay, encoder_info.fps);
            encoder_info.encoder.write_frame(&frame)?;
            Ok(0)
        }
        None => Err(anyhow!("Eecoder不存在!"))
    }
}

pub fn append_bitmap_to_decoder(file: *const c_char, image_data: *const i32, len: i32) -> Result<i32>{
    let pixels = unsafe{ slice::from_raw_parts(image_data, len as usize) };
    let mut pixel_data = Vec::with_capacity(pixels.len() * 4);
    for pixel in pixels{
        let argb = pixel.to_le_bytes();
        //argb转换成rgba
        let rgba_data = [argb[1], argb[2], argb[3], argb[0]];
        pixel_data.extend_from_slice(&rgba_data);
    }
    append_rgba_to_decoder(file, &mut pixel_data)
}


pub fn append_bgr_image_to_decoder(file: *const c_char, bgr_data: *const u8, len: i32) -> Result<i32>{
    let bgr_pixels = unsafe{ slice::from_raw_parts(bgr_data, len as usize) };
    let mut pixel_data = Vec::with_capacity(len as usize/3 * 4);
    for bgr in bgr_pixels.chunks(3){
        //bgr转换成rgba
        let rgba_data = [bgr[1], bgr[2], bgr[3], 255];
        pixel_data.extend_from_slice(&rgba_data);
    }
    append_rgba_to_decoder(file, &mut pixel_data)
}

pub fn append_rgb_image_to_decoder(file: *const c_char, rgb_data: *const u8, len: i32) -> Result<i32>{
    let rgb_pixels = unsafe{ slice::from_raw_parts(rgb_data, len as usize) };
    let mut pixel_data = Vec::with_capacity(len as usize/3 * 4);
    for rgb in rgb_pixels.chunks(3){
        //bgr转换成rgba
        let rgba_data = [rgb[0], rgb[1], rgb[2], 255];
        pixel_data.extend_from_slice(&rgba_data);
    }
    append_rgba_to_decoder(file, &mut pixel_data)
}

pub(crate) fn decode<R: Read>(decoder: Result<Decoder<R>>, overlay: bool, cb: DecodeCallback) -> Result<i32>{
    let mut decoder = decoder?;
    if !overlay{
        while let Some(frame) = decoder.read_next_frame()? {
            //rgba转 argb
            let mut colors = Vec::with_capacity((frame.width * frame.height) as usize * 4);
            for pixel in frame.buffer.chunks(4){
                colors.extend_from_slice(&[pixel[3], pixel[0], pixel[1], pixel[2]]);
            }
            cb(colors.as_ptr(), frame.width as i32, frame.height as i32);
        }
        Ok(0)
    }else{
        //创建一个画布，逐帧叠加绘制
        let mut dt = DrawTarget::new(decoder.width().into(), decoder.height().into());
        
        while let Some(frame) = decoder.read_next_frame()? {
            //gif解析出来的图片是带透明通道的

            //转换为raqote的Image格式
            let img_buf:Vec<u32> = frame.buffer.chunks(4).map(|p|
                //argb
                (p[3] as u32) << 24 | ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | (p[2] as u32)
            ).collect();

            let bitmap = Image {
                width: frame.width as i32,
                height: frame.height as i32,
                data: &img_buf[..],
            };

            //叠加绘制
            dt.draw_image_at(frame.left.into(), frame.top.into(), &bitmap, &DrawOptions::new());
            // get_data方法本身返回的是argb，所以直接转换成指针即可
            cb(dt.get_data().as_ptr() as *const u8, dt.width() as i32, dt.height() as i32);
        }
        Ok(0)
    }
}

pub fn open_decoder_from_bytes(file_data: &[u8]) -> Result<Decoder<&[u8]>>{
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    Ok(options.read_info(file_data)?)
}

pub fn open_decoder_from_file(file: *const c_char) -> Result<Decoder<File>>{
    let file_name = unsafe{ CStr::from_ptr(file) };
    let mut options = gif::DecodeOptions::new();
    options.set_color_output(gif::ColorOutput::RGBA);
    let decoder = options.read_info(File::open(file_name.to_str()?)?)?;
    Ok(decoder)
}