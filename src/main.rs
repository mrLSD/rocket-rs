#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::response::{NamedFile};

#[get("/")]
fn hello() -> &'static str {
    "rocket.rs"
}

// Static files handler
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
