use std::error::Error;

pub type EmptyResult = Result<(), Box<dyn Error>>;
pub type EmptyStaticResult = Result<(), Box<dyn Error + Send + Sync>>;
