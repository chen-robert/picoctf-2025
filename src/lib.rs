#[macro_export]
macro_rules! err {
    ($msg:expr) => {
        Box::<dyn std::error::Error>::from($msg.to_string())
    };
}

pub mod assembler;
pub mod state;
pub mod simulator;
pub mod error; 