//For when I decide to make a web panel

use std::env;
use std::thread;

#[get("/")]
fn index() -> &'static str {
    "Web panel coming soon"
}

pub fn route_network() {
    thread::spawn(|| {
        env::set_var("ROCKET_PORT", env::var("PORT").expect("port"));

        println!("Routing network");

        rocket::ignite().mount("/", routes![index]).launch();
    });
}
