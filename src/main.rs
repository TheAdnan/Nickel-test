#[macro_use] extern crate nickel;
use nickel::{Nickel, HttpRouter};

fn main() {
    let mut server = Nickel::new();

    server.get("/kicin", middleware!(
        "/kicin route"
    ));
    server.get("/user/:id", middleware! { |request|
        format!("This is user: {:?}", request.param("id").unwrap())
    });

    server.listen("127.0.0.1:1992");
}
