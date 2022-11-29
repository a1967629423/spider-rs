use reqwest::Error as ReqwestError;
#[derive(Debug,thiserror::Error)]
pub enum PicaError {
    #[error("reqwest error {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("string error {0}")]
    String(String)
}

pub type PicaResult<T> = Result<T,PicaError>;