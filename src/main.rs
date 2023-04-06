use std::{sync::{mpsc, Arc}, thread::Thread};

use actix_web::{HttpServer, App};
use pcap::Device;
use pcap_rust::*;
// use tokio;

#[tokio::main]
async fn main () -> std::io::Result<()> {
    let (s,r) = mpsc::channel::<String>();
    let mut cap = Device::lookup().unwrap().unwrap().open().unwrap();
    cap.filter("dst port 443", true).unwrap();
    // 创建 tokio pcap 线程
    tokio::spawn(async move {
        while let Ok(packet) = cap.next_packet() {
            // println!("received packet! {:?}", packet);
            s.send("hello".to_string()).unwrap();
        }
    });
    // web 主线程
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(tcp_check_os)
            .app_data(&r)
    }).bind(("127.0.0.1", 8080))?
    .run()
    .await
}