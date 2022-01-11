//use rocket_contrib::templates::Template;
use rocket_dyn_templates::{Template};

#[derive(serde::Serialize,serde::Deserialize)]
struct IndexData<'a>{
    current_user: &'a str,
    layout_name: &'a str,
}


#[get("/")]
pub fn api_webpage () -> Template {
    let ctx = &IndexData{
        current_user: "some random stuff idk inside home.rs",
        layout_name: "template_1"
    };
    Template::render("index", ctx)
}