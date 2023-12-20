#[macro_use] extern crate rocket;

use rocket::tokio::fs::File;
use std::path::Path;

const PREFIX: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/");

#[get("/small")]
async fn small() -> Option<File> {
    let filename = Path::new(PREFIX).join("file-test").join("small.json");
    File::open(&{filename}).await.ok()
}

#[get("/medium")]
async fn medium() -> Option<File> {
    let filename = Path::new(PREFIX).join("file-test").join("medium.json");
    File::open(&{filename}).await.ok()
}

#[get("/large")]
async fn large() -> Option<File> {
    let filename = Path::new(PREFIX).join("file-test").join("large.json");
    File::open(&{filename}).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![small, medium, large])
}