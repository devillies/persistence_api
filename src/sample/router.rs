use rocket;

use crate::connection;
use crate::sample;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount(
            "/posts",
            routes![
                sample::handler::all_post,
                sample::handler::create_post,
                sample::handler::delete_post,
                sample::handler::get_post,
                sample::handler::update_post
            ],
        )
        .launch();
}
