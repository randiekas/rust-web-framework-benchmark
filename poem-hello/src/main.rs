use poem::{
    get, handler, listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server,
};

#[handler]
fn hello() -> String {
    format!("Hello world!")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    
    let app = Route::new().at("/", get(hello)).with(Tracing);
    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .name("hello-world")
        .run(app)
        .await
        
}