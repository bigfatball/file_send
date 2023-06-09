//use actix_files::NamedFile;

use actix::prelude::Stream;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;


mod server;
use self::server::MyWebSocket;
use actix_http;

async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (a,b)=stream.size_hint();
    let c = req.headers();
    println!("{}{:?}",a,b);
    println!("c{:?}",c);
    ws::WsResponseBuilder::new(MyWebSocket::new(),&req,stream)
    .codec(actix_http::ws::Codec::new())
    //.frame_size(10737418240)
    .frame_size(2149580800)
    .protocols(&["A","B"])
    .start()
    
    //start(MyWebSocket::new(), &req, stream)
}

// async fn index() -> impl Responder {
//     NamedFile::open_async("../app/dist/index.html").await.unwrap()
// }


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        
        // .service(
        //     web::resource("/").to(index)
        // )
        .service(
            web::resource("/ws1").route(web::get().to(websocket))
        )
     
        
        .service(
            web::resource("/").route(web::get().to(websocket))
        )
        

    })
    .workers(4)
    .bind(("192.168.248.130", 8080))?
    .run()
    .await



}