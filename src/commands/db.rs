use crate::aof::delete_aof_file;
use crate::errors::ZystError;
use crate::response::ZystResponse;
use crate::types::Db;

pub async fn flush_db(db: &Db) -> Result<ZystResponse, ZystError> {
    db.write().await.clear();
    delete_aof_file().await;
    Ok(ZystResponse::Ok)
}
