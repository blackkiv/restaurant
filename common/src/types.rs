use std::error::Error;

pub type EmptyResult = Result<(), Box<dyn Error + Send + Sync>>;
pub type TypedResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
