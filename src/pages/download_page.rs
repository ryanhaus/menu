use rocket::{ get, http };
use std::io;

use super::get_file;

#[get("/download/<image_id>")]
pub async fn download_page(image_id: String) -> io::Result<(http::ContentType, Vec<u8>)> {
    let image_id = image_id.split(".").collect::<Vec<&str>>()[0].to_string();
    let file = get_file::get_file(image_id)
        .unwrap().1;

    Ok((http::ContentType::Binary, file))
}