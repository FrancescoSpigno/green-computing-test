use poem::{get, handler, listener::TcpListener, web::Path, IntoResponse, Route, Server};

use poem::{endpoint::StaticFilesEndpoint};



#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().nest(
        "/",
        StaticFilesEndpoint::new("file-test")
            .show_files_listing()
    );
    println!("Listening on http://127.0.0.1:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await
}