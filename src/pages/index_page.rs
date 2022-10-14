use handlebars::Handlebars;
use rocket::{ get, http::ContentType };

const INDEX_FILE_SRC: &str = include_str!("templates/img_set.html.hbs");

#[get("/")]
pub fn index_page() -> (ContentType, String) {
    let handlebars = Handlebars::new();
    let data: Option<i32> = None;

    (ContentType::HTML, handlebars.render_template(INDEX_FILE_SRC, &data).unwrap())
}