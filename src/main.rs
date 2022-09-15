mod config;
mod pages;
mod ws_handler;

use rocket::{
    launch, routes, fs, Config,
    data::{ Limits, ToByteUnit }
};

#[launch]
fn rocket() -> _ {
    ws_handler::create_ws_thread();

    let limits = Limits::default()
        .limit("image", 16.megabytes());
        
    let figment = Config::figment()
        .merge((Config::ADDRESS, config::IP_ADDR))
        .merge((Config::PORT, config::HTTP_PORT))
        .merge((Config::LIMITS, limits));

    rocket::custom(figment)
        .mount("/", routes![
            pages::index_page::index_page,
            pages::get_file::get_file,
            pages::view_page::view_page,
            pages::edit_page::edit_page,
            pages::upload_page::upload_page,
            pages::download_page::download_page
        ])
        .mount("/static", fs::FileServer::from("public/"))
}