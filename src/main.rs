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

#[get("/login")]
fn login() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("backend/login/login", &context)
}

#[get("/signup")]
fn signup() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("backend/login/signup", &context)
}

#[get("/forgotpassword")]
fn forgotpassword() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("backend/login/forgot_password", &context)
}

// Static files handler
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[error(404)]
fn error_not_found(_: &Request) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("errors/404", &context)
}

#[error(500)]
fn error_internal(_: &Request) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("errors/500", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            hello, files, login, signup, forgotpassword
        ])
        .attach(Template::fairing())
        .catch(errors![error_not_found, error_internal])
        .launch();
}
