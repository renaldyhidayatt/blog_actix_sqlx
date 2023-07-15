use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    domain::{CreateCategoryRequest, UpdateCategoryRequest},
    service_register::ServiceRegister,
};

#[get("/category")]
async fn get_categories_handler(state: web::Data<ServiceRegister>) -> impl Responder {
    let query_result = state.category_service.get_categories().await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all category items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let category = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": category.len(),
        "category": category
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/category")]
async fn create_category_handler(
    body: web::Json<CreateCategoryRequest>,
    state: web::Data<ServiceRegister>,
) -> impl Responder {
    let query_result = state
        .category_service
        .create_category(&body.name.to_string())
        .await;

    match query_result {
        Ok(category) => {
            let category_response = serde_json::json!({"status": "success", "data": serde_json::json!({"category": category})});

            return HttpResponse::Ok().json(category_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail","message": "Category with that name already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

#[get("/category/{id}")]
async fn get_category_handler(
    path: web::Path<uuid::Uuid>,
    state: web::Data<ServiceRegister>,
) -> impl Responder {
    let category_id = path.into_inner();

    let query_result = state.category_service.get_category(category_id).await;

    match query_result {
        Ok(category) => {
            let category_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "category": category
            })});

            return HttpResponse::Ok().json(category_response);
        }
        Err(_) => {
            let message = format!("Category with ID: {} not found", category_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail","message": message}));
        }
    }
}

#[patch("/category/{id}")]
async fn edit_category_handler(
    path: web::Path<uuid::Uuid>,
    body: web::Json<UpdateCategoryRequest>,
    state: web::Data<ServiceRegister>,
) -> impl Responder {
    let category_id = path.into_inner();

    let query_result = state.category_service.get_category(category_id).await;

    if query_result.is_err() {
        let message = format!("Category with ID: {} not found", category_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let _note = query_result.unwrap();

    let query_result = state
        .category_service
        .update_category(category_id, &body.name.to_string())
        .await;

    match query_result {
        Ok(category) => {
            let category_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "category": category
            })});

            return HttpResponse::Ok().json(category_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": message}));
        }
    }
}

#[delete("/category/{id}")]
async fn delete_category_handler(
    path: web::Path<uuid::Uuid>,
    state: web::Data<ServiceRegister>,
) -> impl Responder {
    let category_id = path.into_inner();

    if let Err(err) = state.category_service.delete_category(category_id).await {
        log::error!("Failed to delete category: {:?}", err);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::NoContent().finish()
}
