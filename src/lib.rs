use jni::{objects::{JObject, JString}, AttachGuard, JavaVM};
use ndk_context::android_context;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Content not available")]
    ContentNotAvailable,
    #[error("JNI error: {0}")]
    JniError(jni::errors::Error),
}

impl From<jni::errors::Error> for Error {
    fn from(error: jni::errors::Error) -> Self {
        Error::JniError(error)
    }
}

pub fn set_text(text: String) -> Result<(), Error> {
    Ok(())
}

pub fn get_text() -> Result<String, Error> {
    let context = AndroidContext::new();
    let vm = context.vm()?;
    let mut env = vm.attach_current_thread()?;
    let clipboard_manager = clipboard_manager(&mut env)?;

    if !env
        .call_method(&clipboard_manager, "hasPrimaryClip", "()Z", &[])?
        .z()?
    {
        return Err(Error::ContentNotAvailable);
    }

    let clip = env
        .call_method(
            clipboard_manager,
            "getPrimaryClip",
            "()Landroid/content/ClipData;",
            &[],
        )?
        .l()?;

    if env.call_method(&clip, "getItemCount", "()I", &[])?.i()? == 0 {
        return Err(Error::ContentNotAvailable);
    }

    let item = env
        .call_method(
            &clip,
            "getItemAt",
            "(I)Landroid/content/ClipData$Item;",
            &[0.into()],
        )?
        .l()?;

    let text = env
        .call_method(item, "getText", "()Ljava/lang/CharSequence;", &[])?
        .l()?;
    let text = JString::from(text);
    let text = env.get_string(&text)?;
    Ok(text.into())
}

pub fn clear() -> Result<(), Error> {
    Ok(())
}

struct AndroidContext(ndk_context::AndroidContext);

impl AndroidContext {
    fn new() -> Self {
        Self(android_context())
    }

    fn vm(&self) -> Result<JavaVM, jni::errors::Error> {
        // SAFETY: Valid pointer guaranteed by the `ndk_context` crate.
        unsafe { jni::JavaVM::from_raw(self.0.vm().cast()) }
    }

    fn context(&self) -> JObject {
        // SAFETY: Valid pointer guaranteed by the `ndk_context` crate.
        unsafe { JObject::from_raw(android_context().context().cast()) }
    }
}

fn clipboard_manager<'attachment>(
    env: &mut AttachGuard<'attachment>,
) -> Result<JObject<'attachment>, Error> {
    let context = AndroidContext::new();
    let clipboard = env.new_string("clipboard")?;

    Ok(env
        .call_method(
            context.context(),
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[(&clipboard).into()],
        )?
        .l()?)
}
