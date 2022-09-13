use rocket::{ get, http };
use std::{ fs, io };
use serde_json::{ Map, Value };

#[get("/file/<file_id>")]
pub fn get_file(file_id: String) -> io::Result<(http::ContentType, Vec<u8>)> {
    // open json file as a reader
    let reader = fs::File::open("data/extensions.json")?;
    let reader = io::BufReader::new(reader);

    // parse the json in the file
    let json: Value = serde_json::from_reader(reader)?;
    let json: &Map<String, Value> = json.as_object().unwrap();

    // read the correct field and generate a content type from it
    let file_type = json
        .get(&file_id).unwrap()
        .as_str().unwrap();
    let file_type = http::ContentType::from_extension(file_type).unwrap();

    // read image contents from the file
    let image_contents = fs::read(format!("data/{}.bin", file_id))?;

    Ok((file_type, image_contents))
}