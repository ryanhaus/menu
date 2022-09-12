use rocket::{ get, http };
use std::{ fs, io };
use serde_json::{ Map, Value };

#[get("/img/<image_id>")]
pub fn get_img_file(image_id: String) -> io::Result<(http::ContentType, Vec<u8>)> {
    // open json file as a reader
    let reader = fs::File::open("data/extensions.json")?;
    let reader = io::BufReader::new(reader);

    // parse the json in the file
    let json: Value = serde_json::from_reader(reader)?;
    let json: &Map<String, Value> = json.as_object().unwrap();

    // read the correct field
    let file_type = json.get(&image_id).unwrap();
    let file_type = file_type.as_str().unwrap();

    let image_contents = fs::read(format!("data/{}.bin", image_id))?;
    let content_type = http::ContentType::from_extension(file_type).unwrap();

    Ok((content_type, image_contents))
}