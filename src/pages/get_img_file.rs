use rocket::{ get, http };
use std::fs;
use std::io::Result;

#[get("/img/<image_id>")]
pub fn get_img_file(image_id: String) -> Result<(http::ContentType, Vec<u8>)> {
    let file_type = fs::read_to_string(format!("data/{}.extension.txt", image_id))?;
    let image_contents = fs::read(format!("data/{}.bin", image_id))?;

    let content_type = http::ContentType::from_extension(&file_type).unwrap();

    Ok((content_type, image_contents))
}