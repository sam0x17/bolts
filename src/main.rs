#[allow(dead_code)]
pub mod router;
use router::*;

pub fn main() {
    println!("hello world");
    let mut router = Router::new();
    router
        .route(Verb::Post, "/this/is/a/:int/;float/#string")
        .unwrap();
    println!("first one ok");
    router.route(Verb::Patch, "/this/is/invalid:ok/").unwrap();
    println!("second one ok");
}
