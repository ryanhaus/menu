use rocket::{ get, http::ContentType};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "img_view.stpl")]
struct ImgViewTemplate {
    monitor_id: String,
    ip_addr: &'static str,
    ws_port: i32
}

#[get("/monitor/<monitor_id>")]
pub fn view_monitor(monitor_id: String) -> (ContentType, String) {
    let ctx = ImgViewTemplate {
        monitor_id: monitor_id,
        ip_addr: crate::config::IP_ADDR,
        ws_port: crate::config::WS_PORT
    };
    
    (ContentType::HTML, ctx.render_once().unwrap())
}