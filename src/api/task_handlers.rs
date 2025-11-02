use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;
use crate::api::dto::{ApiResponse, PaginationParams};
use crate::database::Database;
use crate::errors::{AppError, AppResult};
use crate::models::{CreateTaskRequest, Task, TaskResponse, UpdateTaskRequest};
use crate::utils::validation::validate;

#[rocket::get("/tasks?<page>&<limit>")]
pub async fn get_tasks(
    db: &State<Database>,
    page: Option<u64>,
    limit: Option<u64>,
) -> AppResult<Json<ApiResponse<Vec<TaskResponse>>>> {
    let params = PaginationParams { page, limit };
    let tasks = sqlx::query_as::<_, Task>(
        "SELECT id, title, description, completed, created_at, updated_at 
         FROM tasks 
         ORDER BY created_at DESC 
         LIMIT $1 OFFSET $2"
    )
    .bind(params.limit() as i64)
    .bind(params.offset() as i64)
    .fetch_all(db.pool())
    .await?;

    let responses: Vec<TaskResponse> = tasks.into_iter().map(TaskResponse::from).collect();
    Ok(Json(ApiResponse::success(responses)))
}

#[rocket::get("/tasks/<id>")]
pub async fn get_task(
    db: &State<Database>,
    id: &str,
) -> AppResult<Json<ApiResponse<TaskResponse>>> {
    let id_str = id;
    let id = Uuid::parse_str(id_str)
        .map_err(|_| AppError::BadRequest(format!("ID invalide: {}", id_str)))?;
    
    let task = sqlx::query_as::<_, Task>(
        "SELECT id, title, description, completed, created_at, updated_at 
         FROM tasks 
         WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(db.pool())
    .await?;

    match task {
        Some(task) => Ok(Json(ApiResponse::success(TaskResponse::from(task)))),
        None => Err(AppError::NotFound(format!("Tâche avec l'id {} non trouvée", id))),
    }
}

#[rocket::post("/tasks", data = "<request>")]
pub async fn create_task(
    db: &State<Database>,
    request: Json<CreateTaskRequest>,
) -> AppResult<Json<ApiResponse<TaskResponse>>> {
    let request_data = request.into_inner();
    validate(&request_data)?;
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();

    let task = sqlx::query_as::<_, Task>(
        "INSERT INTO tasks (id, title, description, completed, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)
         RETURNING id, title, description, completed, created_at, updated_at"
    )
    .bind(id)
    .bind(&request_data.title)
    .bind(&request_data.description)
    .bind(false)
    .bind(now)
    .bind(now)
    .fetch_one(db.pool())
    .await?;

    Ok(Json(ApiResponse::success_with_message(
        TaskResponse::from(task),
        "Tâche créée avec succès".to_string(),
    )))
}

#[rocket::put("/tasks/<id>", data = "<request>")]
pub async fn update_task(
    db: &State<Database>,
    id: &str,
    request: Json<UpdateTaskRequest>,
) -> AppResult<Json<ApiResponse<TaskResponse>>> {
    let id_str = id;
    let id = Uuid::parse_str(id_str)
        .map_err(|_| AppError::BadRequest(format!("ID invalide: {}", id_str)))?;
    
    let request_data = request.into_inner();
    validate(&request_data)?;
    // Récupérer la tâche existante
    let existing_task = sqlx::query_as::<_, Task>(
        "SELECT id, title, description, completed, created_at, updated_at 
         FROM tasks 
         WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(db.pool())
    .await?;

    let mut task = match existing_task {
        Some(t) => t,
        None => return Err(AppError::NotFound(format!("Tâche avec l'id {} non trouvée", id))),
    };

    // Mettre à jour les champs fournis
    if let Some(title) = request_data.title {
        task.title = title;
    }
    if request_data.description.is_some() {
        task.description = request_data.description;
    }
    if let Some(completed) = request_data.completed {
        task.completed = completed;
    }
    task.updated_at = chrono::Utc::now();

    // Sauvegarder les modifications
    let updated_task = sqlx::query_as::<_, Task>(
        "UPDATE tasks 
         SET title = $1, description = $2, completed = $3, updated_at = $4
         WHERE id = $5
         RETURNING id, title, description, completed, created_at, updated_at"
    )
    .bind(&task.title)
    .bind(&task.description)
    .bind(task.completed)
    .bind(task.updated_at)
    .bind(id)
    .fetch_one(db.pool())
    .await?;

    Ok(Json(ApiResponse::success_with_message(
        TaskResponse::from(updated_task),
        "Tâche mise à jour avec succès".to_string(),
    )))
}

#[rocket::delete("/tasks/<id>")]
pub async fn delete_task(
    db: &State<Database>,
    id: &str,
) -> AppResult<Json<ApiResponse<()>>> {
    let id_str = id;
    let id = Uuid::parse_str(id_str)
        .map_err(|_| AppError::BadRequest(format!("ID invalide: {}", id_str)))?;
    
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(id)
        .execute(db.pool())
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Tâche avec l'id {} non trouvée", id)));
    }

    Ok(Json(ApiResponse::success_with_message(
        (),
        "Tâche supprimée avec succès".to_string(),
    )))
}

