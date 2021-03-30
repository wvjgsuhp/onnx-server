FROM rust:1.51 as builder
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/onnx-server
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
ENV HOST=0.0.0.0
ENV PORT=8080
ENV PROJECT_ROOT=/usr/src/onnx-server
ENV PROJECT_RELEASE=$PROJECT_ROOT/target/release
ENV MY_LD_LIB_PATH=/usr/lib/x86_64-linux-gnu
ENV LD_LIBRARY_PATH=$MY_LD_LIB_PATH:$LD_LIBRARY_PATH
ENV MODEL_LOCATION=/usr/local/src/fashion_mnist_simple_tf_api.onnx

RUN apt-get update && \
    apt-get install -y build-essential && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder $PROJECT_RELEASE/onnx-server \
    /usr/local/bin/onnx-server
COPY --from=builder $PROJECT_RELEASE/build/onnxruntime-sys-*/out/onnxruntime/onnxruntime-linux-x64-1.6.0/lib/libonnxruntime.so.1.6.0 $MY_LD_LIB_PATH/
COPY ./fashion_mnist_simple_tf_api.onnx $MODEL_LOCATION
COPY ./my_shirt.png /usr/local/src/

CMD ["onnx-server"]
