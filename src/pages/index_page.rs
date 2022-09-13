use rocket::{ get, http::ContentType };

const INDEX_FILE_SRC: &'static str = include_str!("templates/img_set.html");

#[get("/")]
pub fn index_page() -> (ContentType, &'static str) {
    (ContentType::HTML, INDEX_FILE_SRC)
}