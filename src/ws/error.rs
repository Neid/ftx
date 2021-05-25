use tokio_tungstenite::tungstenite;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Tungstenite(tungstenite::Error),
    Serde(serde_json::Error),
}

impl From<tungstenite::Error> for Error {
    fn from(err: tungstenite::Error) -> Error {
        Error::Tungstenite(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Serde(err)
    }
}