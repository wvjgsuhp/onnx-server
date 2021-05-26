use crate::neuralnetwork::*;
use actix_web::{post, web, HttpResponse, Responder};
use onnxruntime::{ndarray::Array, session::Session, tensor::OrtOwnedTensor};
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct AppData {
    session: Arc<Mutex<Session<'static>>>,
    input_shape: Vec<usize>,
}

#[post("/predict")]
async fn predict(app_data: web::Data<AppData>, image_uploaded: web::Json<Image>) -> impl Responder {
    // load image
    let load_img_start = Instant::now();

    let img = Image::load(&image_uploaded.location).into_luma8().to_vec();

    let load_img_duration = load_img_start.elapsed();
    println!("image load time: {:?}", load_img_duration);

    // inference
    let inference_start = Instant::now();

    let mut fashion_mnist_session = app_data.session.lock().unwrap();
    let input_shape: Vec<usize> = app_data.input_shape.to_owned();
    let input = Array::from(img).into_shape(input_shape).unwrap();
    let input_tensors = vec![input];

    let true_inference_start = Instant::now();
    let output: Vec<OrtOwnedTensor<f32, _>> = fashion_mnist_session
        .run(input_tensors)
        .expect("predict failed");
    let true_inference_duration = true_inference_start.elapsed();
    println!("true inference time: {:?}", true_inference_duration);

    let inference_duration = inference_start.elapsed();
    println!("inference time: {:?}", inference_duration);

    HttpResponse::Ok().body(format!("{:?}", output))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let session = init_session();
    let input_shape: Vec<usize> = init_session().inputs[0]
        .dimensions()
        .map(|d| d.unwrap())
        .collect();
    let session = Arc::new(Mutex::new(session));
    let app_data = AppData {
        session: session,
        input_shape: input_shape,
    };
    cfg.data(app_data);
    cfg.service(predict);
}
