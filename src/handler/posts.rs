use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    domain::{CreatePostRequest, UpdatePostRequest},
    service_register::ServiceRegister,
};

#[get("/posts")]
async fn get_posts_handler(state: web::Data<ServiceRegister>) -> impl Responder {
    let query_result = state.posts_service.get_all_posts().await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all posts items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let category = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": category.len(),
        "posts": category
    });
    HttpResponse::Ok().json(json_response)
}

#[get("/posts/{post_id}")]
async fn get_post_handler(
    state: web::Data<ServiceRegister>,
    path: web::Path<uuid::Uuid>,
) -> impl Responder {
    let post_id = path.into_inner();
    let query_result = state.posts_service.get_post(post_id).await;

    if let Err(err) = query_result {
        let message = format!("Something bad happened while fetching post: {}", err);
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    let post = query_result.unwrap();

    if let Some(post) = post {
        let json_response = json!({
            "status": "success",
            "post": post
        });
        HttpResponse::Ok().json(json_response)
    } else {
        let message = format!("Post with ID {} not found", post_id);
        HttpResponse::NotFound().json(json!({"status": "error", "message": message}))
    }
}

#[post("/posts")]
async fn create_post_handler(
    state: web::Data<ServiceRegister>,
    payload: web::Json<CreatePostRequest>,
) -> impl Responder {
    let create_post_result = state
        .posts_service
        .create_post(
            &payload.title,
            &payload.body,
            payload.category_id,
            payload.user_id,
            &payload.user_name,
        )
        .await;

    if let Err(err) = create_post_result {
        let message = format!("Something bad happened while creating post: {}", err);
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    let created_post = create_post_result.unwrap();

    let json_response = json!({
        "status": "success",
        "post": created_post
    });

    HttpResponse::Created().json(json_response)
}

#[patch("/posts/{post_id}")]
async fn update_post_handler(
    state: web::Data<ServiceRegister>,
    path: web::Path<uuid::Uuid>,
    payload: web::Json<UpdatePostRequest>,
) -> impl Responder {
    let post_id = path.into_inner();

    let update_post_result = state
        .posts_service
        .update_post(
            post_id,
            &payload.title,
            &payload.body,
            payload.category_id,
            payload.user_id,
            &payload.user_name,
        )
        .await;

    if let Err(err) = update_post_result {
        let message = format!("Something bad happened while updating post: {}", err);
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    let updated_post = update_post_result.unwrap();

    if let Some(updated_post) = updated_post {
        let json_response = json!({
            "status": "success",
            "post": updated_post
        });
        HttpResponse::Ok().json(json_response)
    } else {
        let message = format!("Post with ID {} not found", post_id);
        HttpResponse::NotFound().json(json!({"status": "error", "message": message}))
    }
}

#[delete("/posts/{post_id}")]
async fn delete_post_handler(
    state: web::Data<ServiceRegister>,
    path: web::Path<uuid::Uuid>,
) -> impl Responder {
    let post_id = path.into_inner();
    let delete_post_result = state.posts_service.delete_post(post_id).await;

    if let Err(err) = delete_post_result {
        let message = format!("Something bad happened while deleting post: {}", err);
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    HttpResponse::NoContent().finish()
}
