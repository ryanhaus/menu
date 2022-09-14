use rocket::{ get, http::ContentType };
use handlebars::Handlebars;

#[derive(serde::Serialize)]
struct ImgEditTemplate {
    monitor_id: String
}

const MONITOR_EDIT_SRC: &'static str = include_str!("templates/edit.html.hbs");

#[get("/edit/<monitor_id>")]
pub fn edit_page(monitor_id: String) -> (ContentType, String) {
    let data = ImgEditTemplate {
        monitor_id: monitor_id
    };

    let handlebars = Handlebars::new();
    
    (ContentType::HTML, handlebars.render_template(MONITOR_EDIT_SRC, &data).unwrap())
}
