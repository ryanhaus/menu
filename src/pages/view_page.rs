use rocket::{ get, http::ContentType};
use handlebars::Handlebars;

#[derive(serde::Serialize)]
struct ImgViewTemplate {
    monitor_id: String,
    ip_addr: &'static str,
    ws_port: i32
}

const MONITOR_VIEW_SRC: &'static str = include_str!("templates/view.html.hbs");


#[get("/monitor/<monitor_id>")]
pub fn view_page(monitor_id: String) -> (ContentType, String) {
    let data = ImgViewTemplate {
        monitor_id: monitor_id,
        ip_addr: crate::config::IP_ADDR,
        ws_port: crate::config::WS_PORT
    };

    let handlebars = Handlebars::new();
    
    (ContentType::HTML, handlebars.render_template(MONITOR_VIEW_SRC, &data).unwrap())
}