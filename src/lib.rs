use actix_web::{get, HttpRequest, Responder, HttpResponse, http::header};
use serde::{Deserialize, Serialize};

#[get("/")]
pub async fn hello(req: HttpRequest) -> impl Responder {
    for (k,v) in req.headers() {
        println!("{:?}:{:?}", k, v)
    }
    format!("{:?}", req.headers())
}

#[get("/tcp/checkOs")]
pub async fn tcp_check_os(req: HttpRequest) -> impl Responder {
    let ua = req.headers().get(header::USER_AGENT);
    
    HttpResponse::Ok().json(ResponseData::new("200", "200", None as Option<i32>))
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ResponseData<T> {
    msg: String,
    code: String,
    data: Option<T>
}

impl<T> ResponseData<T> {
    pub fn new(code:&str, msg: &str, data: Option<T>) -> Self{
        ResponseData {
            msg: msg.to_string(),
            code: code.to_string(),
            data
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CheckOsResult {
    check_safe: bool,
    mtu: i32,
    src_ip: String,
    tcp_os: String,
}