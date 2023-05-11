use actix_web::web;
pub mod story;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(story::create_todo)
        .service(story::get_todo)
        .service(story::get_todos)
        .service(story::update_todo)
        .service(story::delete_todo);

    conf.service(scope);
}
