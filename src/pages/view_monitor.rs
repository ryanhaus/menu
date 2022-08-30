use rocket::{ get, http::ContentType};
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "img_view.stpl")]
struct ImgViewTemplate {
    monitor_id: String
}

#[get("/monitor/<monitor_id>")]
pub fn view_monitor(monitor_id: String) -> (ContentType, String) {
    let ctx = ImgViewTemplate { monitor_id: monitor_id };
    (ContentType::HTML, ctx.render_once().unwrap())
}