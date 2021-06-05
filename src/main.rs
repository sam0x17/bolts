#[allow(dead_code)]
pub mod router;
use router::*;

pub fn main() {
    println!("hello world");
    let mut router = Router::new();
    router
        .route(HttpVerb::Post, "/this/is/a/:int/;float/#string/cool/test/")
        .unwrap();
    println!("first one ok");
    router
        .route(HttpVerb::Patch, "/this/is/invalid:ok/")
        .unwrap();
    println!("second one ok");
}
