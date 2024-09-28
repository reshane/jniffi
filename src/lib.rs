use jni::JNIEnv;
use jni::objects::{
    JClass,
    JString,
};
use jni::sys::jstring;

mod image_handler;

#[no_mangle]
pub extern "system" fn Java_HelloWorld_hello<'local>(
    mut env: JNIEnv<'local>,
    class: JClass<'local>,
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
