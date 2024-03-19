use actix_web::{get, web, App, HttpResponse, HttpServer};

use std::net::{IpAddr, UdpSocket, TcpListener};

fn ipv4_address() -> Result<IpAddr, String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
    socket.connect("8.8.8.8:80").map_err(|e| e.to_string())?;
    let local_addr = socket.local_addr().map_err(|e| e.to_string())?;
    let ip_addr = local_addr.ip();
    Ok(ip_addr)
}

#[get("/")]
async fn index() -> HttpResponse {
    let ip_address = ipv4_address().unwrap();
    HttpResponse::Ok().json(ip_address)
}

pub async fn start_actix_web() {
    tokio::task::spawn(async {
        let app = move || {
            App::new().service(index)
        };
    
        // Bind the server to a TCP listener
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind TCP listener");
    
        // Start the Actix Web server
        HttpServer::new(app)
            .listen(listener.into())
            .expect("Failed to start Actix Web server")
            .run()
            .await
            .expect("Failed to run Actix Web server");
    });
}