#[macro_use] extern crate rocket;
//use rocket_contrib::templates::Template;
use rocket_dyn_templates::{Template};

//use rocket::http::Status;
use rocket::{Rocket, Build};
mod home;

#[catch(404)]
fn error_404() -> &'static str {
    "Shit done broke."
}

#[get("/testfunc")]
fn testingfunc() -> &'static str {
    "testing func!"
}

fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![home::api_webpage])
        .attach(Template::fairing())
        .mount("/testing", routes![testingfunc])
}

#[rocket::main]
async fn main() {
    println!("starting backend");
    rocket().launch().await;
}