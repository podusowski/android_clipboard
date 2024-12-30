use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("JNI error")]
    JniError,
}

pub fn set_text(text: String) -> Result<(), Error> {
    Ok(())
}

pub fn get_text() -> Result<String, Error> {
    Ok("".to_string())
}

pub fn clear() -> Result<(), Error> {
    Ok(())
}
