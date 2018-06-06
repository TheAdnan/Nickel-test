#[macro_use] extern crate nickel;
extern crate rustc_serialize;
use nickel::{Nickel, HttpRouter, JsonBody};

#[derive(RustcDecodable, RustcEncodable)]
struct User{
    name: String,
    username: String,
    mail: String
}
fn main() {
    let mut server = Nickel::new();

    server.get("/kicin", middleware!(
        "/kicin route"
    ));
    server.get("/user/:id", middleware! { |request|
        format!("This is user: {:?}", request.param("id").unwrap())
    });

    server.post("/user/view", middleware!{
        |req, res|
        let user = req.json_as::<User>().unwrap();
        format!("Oh hi {}. This is your username {} and email {}", user.name, user.username, user.mail);
    });

    server.listen("127.0.0.1:1992");
}
