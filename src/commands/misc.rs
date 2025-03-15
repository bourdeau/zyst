use crate::errors::ZystError;
use crate::response::ZystResponse;

pub async fn pong() -> Result<ZystResponse, ZystError> {
    Ok(ZystResponse::SimpleString("PONG".to_string()))
}

pub async fn docs() -> Result<ZystResponse, ZystError> {
    Ok(ZystResponse::SimpleString(
        "DOCS is not implemented yet".to_string(),
    ))
}

pub async fn client() -> Result<ZystResponse, ZystError> {
    Ok(ZystResponse::SimpleString(
        "CLIENT SETINFO is not implemented yet".to_string(),
    ))
}
