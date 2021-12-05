mod jni_graphics;

use std::fmt::{Display, self};

use image::RgbaImage;
use jni::{objects::{JClass, JObject, JValue}, sys::{jbyteArray, jobject}};
use jni::sys::jint;
use jni::{JNIEnv, JavaVM};
use anyhow::{anyhow, Result};
use log::{info, error};

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
	let mut frames_obj = None;
	let result = (||->Result<()> {
		let data = env.convert_byte_array(file_data)?;
		
        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);
        // Read the file header
        let mut decoder = options.read_info(data.as_slice())?;
        let mut frames = vec![];
        while let Some(frame) = decoder.read_next_frame().unwrap() {
            // frame.delay;
            if let Some(image) = RgbaImage::from_raw(frame.width as u32, frame.height as u32, frame.buffer.to_vec()){
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
                        JValue::from(rgba_image_to_java_bitmap(&env, &image)?),
                    ],
                )?;
                frames.push(frame_obj);
            }
        }

		let frame_array = env.new_object_array(frames.len() as i32, &"run/ccfish/android/gif/Frame", JObject::null())?;
        for (idx, frame) in frames.into_iter().enumerate(){
            env.set_object_array_element(frame_array, idx as i32, frame)?;
        }
		frames_obj = Some(frame_array);
		Ok(())
	})();

	if result.is_err(){
		let err = result.err();
		error!("{:?}", &err);
		let _ = env.throw_new("java/lang/Exception", format!("{:?}", err));
		JObject::null().into_inner()
	}else{
		frames_obj.unwrap()
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