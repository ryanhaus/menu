use rocket::{ post, form, fs as rfs, response };
use std::fs;
use std::io::Result;
use crate::ws_handler;

#[derive(form::FromForm)]
#[derive(Debug)]
pub struct ImageUploadData<'a> {
    monitor_id: String,
    monitor_image: rfs::TempFile<'a>
}

#[post("/upload_image", data="<form_data>")]
pub async fn upload_image(mut form_data: form::Form<ImageUploadData<'_>>) -> Result<response::Redirect> {
    let image_file_name = form_data.monitor_id.clone();
    let file_destination = format!("data/{}.bin", image_file_name);

    form_data.monitor_image.move_copy_to(file_destination).await?;
    
    let image_file_type = form_data.monitor_image
        .content_type().unwrap()
        .extension().unwrap()
        .to_string();
    fs::write(format!("data/{}.extension.txt", image_file_name), image_file_type)?;

    unsafe {
        println!("{:#?}", ws_handler::SOCKETS_IN_ROOMS);

        for socket_pref in &ws_handler::SOCKETS_IN_ROOMS {
            let socket = &**socket_pref;
            println!("{:?}, {:?}", (socket_pref, socket), (&image_file_name, &socket.room));
            if &image_file_name == &socket.room {
                if !socket.out.send("{\"message_type\":\"RELOAD\"}").is_err() { continue }
            }
        }
    }

    Ok(response::Redirect::to("/"))
}