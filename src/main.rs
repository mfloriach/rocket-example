use rocket::{launch, routes};
#[macro_use]
extern crate rocket;
mod endpoints;
mod models;
mod schema;
mod services;
use rocket::Request;

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("I couldn't find '{}'. Try something else?", req.uri())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![internal_error, not_found])
        .mount(
            "/users",
            routes![
                endpoints::get_posts,
                endpoints::get_post,
                endpoints::create_post,
                endpoints::delete_post,
                endpoints::update_post,
            ],
        )
}
