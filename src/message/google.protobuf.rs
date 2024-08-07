// This file is @generated by prost-build.
/// --------------------------------------------------------------
/// This file was generated by `protobuf_zmq_rust_generator` crate
/// DO NOT MODIFY DIRECTLY
/// --------------------------------------------------------------
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use futures::future::BoxFuture;
use futures::{FutureExt, TryFutureExt};
use prost::Message;
use tokio::task;
use zmq::SocketType;
fn create_socket(path: &str, socket_type: SocketType) -> zmq::Socket {
    let context = zmq::Context::new();
    let socket = context.socket(socket_type).unwrap();
    let protocol = "ipc://";
    create_path_if_not_exists(path);
    let endpoint = format!("{}{}", protocol, path);
    socket.bind(&endpoint).unwrap();
    socket
}
fn create_path_if_not_exists(path_str: &str) {
    let path = std::path::Path::new(path_str);
    let path1 = path.parent().unwrap();
    if !path1.exists() {
        std::fs::create_dir_all(path1).unwrap();
    }
}
