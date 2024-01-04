#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> String {
    format!("Hello world!")
}

#[launch]
fn rocket() -> _ {

    let config = rocket::Config {
        port: 8080,
        log_level: rocket::config::LogLevel::Off,
        ..rocket::Config::debug_default()
    };

    rocket::custom(&config)
        .mount("/", routes![hello])

}