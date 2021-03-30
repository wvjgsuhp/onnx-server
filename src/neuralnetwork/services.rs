use crate::utils;
use lazy_static::lazy_static;
use onnxruntime::{environment::Environment, session::Session};
use onnxruntime::{GraphOptimizationLevel, LoggingLevel};

lazy_static! {
    static ref ORT_ENV: Environment = {
        Environment::builder()
            .with_name("fashion_mnist")
            .with_log_level(LoggingLevel::Verbose)
            .build()
            .expect("Failed to initialize onnxruntime env")
    };
}

pub fn init_env() {
    lazy_static::initialize(&ORT_ENV);
}

pub fn init_session() -> Session<'static> {
    let onnx_path = utils::get_env("MODEL_LOCATION");
    ORT_ENV
        .new_session_builder()
        .expect("new_session_builder failed")
        .with_optimization_level(GraphOptimizationLevel::Basic)
        .expect("with_optimization_level failed")
        .with_number_threads(1)
        .expect("with_number_threads failed")
        .with_model_from_file(onnx_path)
        .expect("with_model_from_file failed")
}
