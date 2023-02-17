use std::{slice, ffi::{c_char, c_int}};
use log::{LevelFilter, error};

mod coder;

fn init_log(){
    android_logger::init_once(android_logger::Config::default().with_max_level(LevelFilter::Info));
}

//解析内存中的GIF图片
#[no_mangle]
pub extern fn decode_bytes(file_data: *const u8, len: i32, overlay: bool, cb: coder::DecodeCallback) -> i32{
    init_log();
    let data= unsafe{ slice::from_raw_parts(file_data, len as usize) };
	coder::decode(coder::open_decoder_from_bytes(data), overlay, cb)
    .map_err(|err|{
        error!("{:?}", err);
        err
    })
    .unwrap_or(-1)
}

//解析GIF图片文件
#[no_mangle]
pub extern fn decode_file(file: *const c_char, overlay: bool, cb: coder::DecodeCallback) -> i32{
    init_log();
	coder::decode(coder::open_decoder_from_file(file), overlay, cb)
    .map_err(|err|{
        error!("{:?}", err);
        err
    })
    .unwrap_or(-1)
}

//创建GIF图片编码器
#[no_mangle]
pub extern fn create_encoder(file: *const c_char, repeat: bool, width: c_int, height: c_int, fps:c_int) -> i32{
    init_log();
    coder::create_encoder(file, repeat, width, height, fps)
    .map_err(|err|{
        error!("{:?}", err);
        err
    }).unwrap_or(-1)
}

//将图片文杰写入GIF
#[no_mangle]
pub extern fn append_file_to_encoder(file: *const c_char, image_file: *const c_char) -> i32{
    init_log();
    coder::append_file_to_encoder(file, image_file)
    .map_err(|err|{
        error!("{:?}", err);
        err
    }).unwrap_or(-1)
}

//将图片文件字节写入GIF
#[no_mangle]
pub extern fn append_file_bytes_to_encoder(file: *const c_char, image_data: *const u8, len: i32) -> i32{
    init_log();
    // info!("append_file_bytes_to_encoder: image_data={:?} len={len}", image_data);
    coder::append_file_data_to_encoder(file, image_data, len)
    .map_err(|err|{
        error!("{:?}", err);
        err
    }).unwrap_or(-1)
}

//将ARGB位图写入GIF
#[no_mangle]
pub extern fn append_bitmap_to_encoder(file: *const c_char, image_data: *const i32, len: i32) -> i32{
    init_log();
    coder::append_bitmap_to_decoder(file, image_data, len)
    .map_err(|err|{
        error!("{:?}", err);
        err
    }).unwrap_or(-1)
}

//将BGR原始数据写入GIF文件中
#[no_mangle]
pub extern fn append_bgr_image_to_encoder(file: *const c_char, bgr_data: *const u8, len: i32) -> i32{
    init_log();
    // info!("append_bgr_image_to_encoder: bgr_data={:?} len={len}", bgr_data);
    coder::append_bgr_image_to_decoder(file, bgr_data, len)
    .map_err(|err|{
        error!("{:?}", err);
        err
    }).unwrap_or(-1)
}

//将RGB原始数据写入GIF文件中
#[no_mangle]
pub extern fn append_rgb_image_to_encoder(file: *const c_char, rgb_data: *const u8, len: i32) -> i32{
    init_log();
    // info!("append_rgb_image_to_encoder: rgb_data={:?} len={len}", rgb_data);
    coder::append_rgb_image_to_decoder(file, rgb_data, len)
    .map_err(|err|{
        error!("{:?}", err);
        err
    }).unwrap_or(-1)
}

//结束GIF编码
#[no_mangle]
pub extern fn close_decoder(file: *const c_char) -> i32{
    init_log();
    coder::close_decoder(file)
    .map_err(|err|{
        error!("{:?}", err);
        err
    }).unwrap_or(-1)
}