use crate::neuralnetwork::*;
use actix_web::{post, web, HttpResponse, Responder};
use onnxruntime::{ndarray::Array, session::Session, tensor::OrtOwnedTensor};
use std::sync::{Arc, Mutex};

#[post("/predict")]
async fn predict(
    session: web::Data<Arc<Mutex<Session<'static>>>>,
    image_uploaded: web::Json<Image>,
) -> impl Responder {
    let mut fashion_mnist_session = session.lock().unwrap();

    let img = Image::load(&image_uploaded.location).into_luma8().to_vec();
    // println!("{:?}", img);

    let input_shape: Vec<usize> = fashion_mnist_session.inputs[0]
        .dimensions()
        .map(|d| d.unwrap())
        .collect();
    let input = Array::from(img).into_shape(input_shape).unwrap();
    let input_tensors = vec![input];

    let output: Vec<OrtOwnedTensor<f32, _>> = fashion_mnist_session
        .run(input_tensors)
        .expect("predict failed");

    HttpResponse::Ok().body(format!("{:?}", output))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let session = Arc::new(Mutex::new(init_session()));

    cfg.data(session);
    cfg.service(predict);
}
