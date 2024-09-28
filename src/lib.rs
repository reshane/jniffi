use jni::JNIEnv;
use jni::objects::{
    JClass,
    JString,
    JByteArray,
};
use jni::sys::jstring;
use image::imageops::FilterType;

mod image_handler;

#[no_mangle]
pub extern "system" fn Java_HelloWorld_hello<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    input: JString<'local>
    ) -> jstring {
    let input: String = env.get_string(&input)
        .expect("Could not get java string from env")
        .into();

    let output = env.new_string(
        format!("Hello, {}", input)
        ).expect("Could not creat java string!");

    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_HelloWorld_helloBytes<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    input: JByteArray<'local>
    ) -> JByteArray<'local> {
    let input = env.convert_byte_array(&input)
        .expect("Could not get java byte[] from env");

    let image = image_handler::get_image_from_bytes(input.to_vec());
    let resized = image_handler::resize_dynamic_image(240, 240, image);
    let bytes: Vec<u8> = image_handler::get_png_bytes(resized);

    let output = env.byte_array_from_slice(&bytes)
        .expect("Could not create java byte[]!");

    output
}


