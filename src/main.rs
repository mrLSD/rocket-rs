#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use rocket::Request;
use rocket::response::{NamedFile};
use rocket_contrib::Template;

#[get("/")]
fn hello() -> &'static str {
    "rocket.rs"
}

// Static files handler
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[error(404)]
fn error_not_found(_: &Request) -> Template {
    let mut map: HashMap<&str, &str> = HashMap::new();
    // map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello, files])
        .attach(Template::fairing())
        .catch(errors![error_not_found])
        .launch();
}
