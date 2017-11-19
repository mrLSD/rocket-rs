#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use rocket::Request;
use rocket::response::{NamedFile, Redirect};
use rocket_contrib::Template;
use rocket::request::{Form};
use rocket::http::RawStr;

#[derive(FromForm)]
struct LoginForm<'r> {
    username: &'r RawStr,
}

#[get("/")]
fn hello() -> &'static str {
    "rocket.rs"
}

#[get("/login")]
fn get_login() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("backend/login/login", &context)
}

#[post("/login", data = "<login_form>")]
fn post_login<'a>(login_form: Form<'a, LoginForm<'a>>) -> Result<Redirect, String> {
    let _ = login_form.get();
    Ok(Redirect::to("/"))
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
fn error_404(_: &Request) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("errors/404", &context)
}

#[error(422)]
fn error_422(_: &Request) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("errors/422", &context)
}

#[error(500)]
fn error_500(_: &Request) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("errors/500", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
            hello, files, get_login, post_login, signup, forgotpassword
        ])
        .attach(Template::fairing())
        .catch(errors![
            error_404, error_422, error_500
        ])
        .launch();
}
