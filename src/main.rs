mod pages;
mod ws_handler;

use rocket::{ launch, routes, fs };

#[launch]
fn rocket() -> _ {
    ws_handler::create_ws_thread();

    rocket::build()
        .mount("/", routes![
            pages::index::index,
            pages::get_img_file::get_img_file,
            pages::view_monitor::view_monitor,
            pages::upload_image::upload_image
        ])
        .mount("/static", fs::FileServer::from("public/"))
}