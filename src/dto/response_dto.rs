use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseDTO<T> {
    pub status: String,
    pub data: T,
}
