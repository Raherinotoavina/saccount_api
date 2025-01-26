use serde::Serialize;

#[derive(Serialize)]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Serialize)]
pub struct ResponseDTO<T> {
    pub status: ResponseStatus,
    pub data: T,
}
