mod jni_graphics;

use std::fmt::{Display, self};

use image::RgbaImage;
use jni::{objects::{JClass, JObject, JValue}, sys::{jbyteArray, jobject}};
use jni::sys::jint;
use jni::{JNIEnv, JavaVM};
use anyhow::{anyhow, Result};
use log::{info, error};
use raqote::{DrawTarget, Image, DrawOptions};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct GifError;

impl Display for GifError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (&format!("{:?}", self)).fmt(f)
    }
}

//JNI加载完成
#[no_mangle]
pub extern "C" fn JNI_OnLoad(_jvm: JavaVM, _reserved: *mut std::ffi::c_void) -> jint {
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Info));
    info!("JNI_OnLoad.");
    jni::sys::JNI_VERSION_1_6
}

//解析GIF图片
#[no_mangle]
pub extern fn Java_run_ccfish_android_gif_Gif_decode<'a>(env: JNIEnv, _activity: JClass, file_data: jbyteArray) -> jobject{
	let mut decode_obj = None;
	let result = (||->Result<()> {
		let data = env.convert_byte_array(file_data)?;
		
        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);
        // Read the file header
        let mut decoder = options.read_info(data.as_slice())?;
        let mut frames = vec![];

        //创建一个画布，逐帧绘制
        let mut dt = DrawTarget::new(decoder.width().into(), decoder.height().into());
        
        while let Some(frame) = decoder.read_next_frame().unwrap() {
            //gif解析出来的图片是带透明通道的
            if let Some(image) = RgbaImage::from_raw(frame.width as u32, frame.height as u32, frame.buffer.to_vec()){
                //转换为raqote的Image格式
                let img_buf:Vec<u32> = image.pixels().map(|p|
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

                //截取当前帧
                let mut out_buf = Vec::with_capacity((dt.width() * dt.height() * 4) as usize);

                // bgra格式转换成 rgba
                dt.get_data_u8().chunks(4).for_each(|pd|{
                    //bgra
                    out_buf.extend_from_slice(&[pd[2], pd[1], pd[0], pd[3]]);
                });
                
                //生成rgba图片
                if let Some(current_frame) = RgbaImage::from_vec(dt.width() as u32, dt.height() as u32, out_buf){
                    let frame_obj = env.new_object(
                        "run/ccfish/android/gif/Frame",
                        "(IIIZIIIIZLandroid/graphics/Bitmap;)V",
                        &[
                            JValue::from(frame.delay as i32),
                            JValue::from(match frame.dispose{
                                gif::DisposalMethod::Any => 0i32,
                                gif::DisposalMethod::Keep => 1,
                                gif::DisposalMethod::Background => 2,
                                gif::DisposalMethod::Previous => 3
                            }),
                            JValue::from(frame.transparent.map(|v| v as i32).unwrap_or(-1)),
                            JValue::from(frame.needs_user_input),
                            JValue::from(frame.top as i32),
                            JValue::from(frame.left as i32),
                            JValue::from(frame.width as i32),
                            JValue::from(frame.height as i32),
                            JValue::from(frame.interlaced),
                            JValue::from(rgba_image_to_java_bitmap(&env, &current_frame)?),
                        ],
                    )?;
                    frames.push(frame_obj);
                }
            }
        }

        //创建 Frame 数组
		let frame_array = env.new_object_array(frames.len() as i32, &"run/ccfish/android/gif/Frame", JObject::null())?;
        for (idx, frame) in frames.into_iter().enumerate(){
            env.set_object_array_element(frame_array, idx as i32, frame)?;
        }
        
        //创建DecodeGif
		decode_obj = Some(env.new_object(
            "run/ccfish/android/gif/DecodeGif",
            "(II[Lrun/ccfish/android/gif/Frame;)V",
            &[
                JValue::from(decoder.width() as i32),
                JValue::from(decoder.height() as i32),
                JValue::from(frame_array),
            ],
        )?);
		Ok(())
	})();

	if result.is_err(){
		let err = result.err();
		error!("{:?}", &err);
		let _ = env.throw_new("java/lang/Exception", format!("{:?}", err));
		JObject::null().into_inner()
	}else{
		decode_obj.unwrap().into_inner()
	}
}

/// RgbaImage to Bitmap
fn rgba_image_to_java_bitmap<'a>(env: &'a JNIEnv, image: &RgbaImage) -> Result<JObject<'a>> {
    //创建java的Bitmap对象，复制新的图像数据
    let bitmap:JObject<'a> = jni_graphics::create_java_bitmap_argb8888(env, image.width() as i32, image.height() as i32)?;
    jni_graphics::lock_bitmap(&env, &bitmap, |info, pixels|{
        //只支持argb888格式
        if info.format != jni_graphics::ANDROID_BITMAP_FORMAT_RGBA_8888{
            Err(anyhow!("图片格式只支持RGBA_8888!"))
        }else{
            //复制图像数据
            let rowbytes = image.width() as usize * 4;
            let img_data = image.as_raw();
            for (y, row) in pixels.chunks_mut(info.stride as usize).enumerate(){
                let start = rowbytes * y;
                row[0..rowbytes].copy_from_slice(&img_data[start..start+rowbytes]);
            }
            Ok(())
        }
    })?;
    Ok(bitmap)
}