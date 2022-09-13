use rocket::{ post, form, fs as rfs, response };
use std::{ fs, io };
use serde_json::{ Value, Map };
use crate::ws_handler;

#[derive(form::FromForm)]
#[derive(Debug)]
pub struct ImageUploadData<'a> {
    monitor_id: String,
    monitor_image: rfs::TempFile<'a>
}

#[post("/upload_image", data="<form_data>")]
pub async fn upload_page(mut form_data: form::Form<ImageUploadData<'_>>) -> io::Result<response::Redirect> {
    let image_file_name = form_data.monitor_id.clone();
    let file_destination = format!("data/{}.bin", image_file_name);

    form_data.monitor_image.move_copy_to(file_destination).await?;
    
    let image_file_type = form_data.monitor_image
        .content_type().unwrap()
        .extension().unwrap()
        .to_string();

    // open json file as a reader
    let reader = fs::File::open("data/extensions.json")?;
    let reader = io::BufReader::new(reader);

    // parse the json in the file
    let mut json: Value = serde_json::from_reader(reader)?;
    let json: &mut Map<String, Value> = json.as_object_mut().unwrap();

    // update the correct field and write to the file
    json.insert(image_file_name.clone(), serde_json::json!(image_file_type));
    fs::write("data/extensions.json", serde_json::to_string(&json).unwrap())?;

    unsafe {
        for socket in &ws_handler::SOCKETS_IN_ROOMS {
            let socket = &**socket;
            
            if &image_file_name == &socket.room {
                if !socket.out.send("{\"message_type\":\"RELOAD\"}").is_err() { continue }
            }
        }
    }

    Ok(response::Redirect::to("/"))
}