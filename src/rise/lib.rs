use std::fmt;

#[derive(Debug, Clone)]
pub struct RISEError;



type Result<T> = std::result::Result<T, RISEError>;

pub mod core;

pub mod graphics;