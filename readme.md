# Inference Via `actix-web` + `onnxruntime` <!-- omit in toc -->

This project demonstrates an inference using trained neural network via REST api in [`rust`](https://www.rust-lang.org/) with [`actix-web`](https://actix.rs/) and [`onnxruntime`](https://docs.rs/onnxruntime/0.0.11/onnxruntime/).

- [Build](#build)
- [Run](#run)
- [Test](#test)

## Build

```bash
docker build -t onnx-server .
```

## Run

```bash
docker run -p 8080:8080 -it --rm onnx-server
```

## Test

```bash
curl --location --request POST '0.0.0.0:8080/predict' \
--header 'Content-Type: application/json' \
--data-raw '{
    "location": "/usr/local/src/my_shirt.png"
}'
```
