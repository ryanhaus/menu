use rocket::{ get, http::ContentType };
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "img_set.stpl")]
struct ImgSetTemplate {
    monitor_count: i32
}

#[get("/")]
pub fn index() -> (ContentType, String) {
    let ctx = ImgSetTemplate { monitor_count: 5 };
    (ContentType::HTML, ctx.render_once().unwrap())
}