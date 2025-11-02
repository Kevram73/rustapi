use crate::api::{handlers, task_handlers};

pub fn get_routes() -> Vec<rocket::Route> {
    rocket::routes![
        handlers::health_check,
        handlers::root,
        task_handlers::get_tasks,
        task_handlers::get_task,
        task_handlers::create_task,
        task_handlers::update_task,
        task_handlers::delete_task,
    ]
}

