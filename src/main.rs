#[macro_use]
extern crate nickel;
extern crate rustc_serialize;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

use nickel::{Nickel, HttpRouter, JsonBody, Response, Request, MediaType, MiddlewareResult};
use nickel::status::StatusCode::{self};


//MongoDB
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::error::Result as MongoResult;

//bson
use bson::{Bson, Document};
use bson::oid::ObjectId;

//rustc
use rustc_serialize::json::{Json, ToJson};

//Profile structure
#[derive(RustcDecodable, RustcEncodable)]
struct Profile{
    name: String,
    username: String,
    email: String
}

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/home", middleware! { |request, response|
        format!("<h1>Welcome!</h1><div><p>My name is <a href='http://theadnan.github.io' target='_blank'>Adnan</a></p></div>")
    });

    router.get("/users", middleware! { |request, response|
        format!("List of users")
    });

    router.post("/users/new", middleware! { |request, response|

        format!("Create a new user")

      });

    server.utilize(router);
    server.listen("127.0.0.1:1992");
}
