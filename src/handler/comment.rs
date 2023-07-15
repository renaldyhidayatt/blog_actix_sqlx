use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    domain::{CreateCommentRequest, UpdateCommentRequest},
    service_register::ServiceRegister,
};

#[get("/comments")]
async fn get_comments_handler(state: web::Data<ServiceRegister>) -> impl Responder {
    let query_result = state.comment_service.get_comments().await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching comments";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    let comments = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": comments.len(),
        "comments": comments
    });

    HttpResponse::Ok().json(json_response)
}

#[get("/comments/{id}")]
async fn get_comment_handler(
    state: web::Data<ServiceRegister>,
    path: web::Path<uuid::Uuid>,
) -> impl Responder {
    let comment_id = path.into_inner();

    let query_result = state.comment_service.get_comment(comment_id).await;

    match query_result {
        Ok(comment) => {
            if let Some(comment) = comment {
                let json_response = serde_json::json!({
                    "status": "success",
                    "comment": comment
                });

                HttpResponse::Ok().json(json_response)
            } else {
                let message = format!("Comment with ID '{}' not found", comment_id);
                HttpResponse::NotFound().json(json!({"status": "fail", "message": message}))
            }
        }
        Err(err) => {
            let message = format!("Error while fetching comment: {:?}", err);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": message}))
        }
    }
}

#[post("/comments")]
async fn create_comment_handler(
    state: web::Data<ServiceRegister>,
    payload: web::Json<CreateCommentRequest>,
) -> impl Responder {
    let result = state
        .comment_service
        .create_comment(
            payload.id_post_comment,
            &payload.user_name_comment,
            &payload.comment,
        )
        .await;

    match result {
        Ok(comment) => {
            let json_response = serde_json::json!({
                "status": "success",
                "comment": comment
            });

            HttpResponse::Ok().json(json_response)
        }
        Err(err) => {
            let message = format!("Error while creating comment: {:?}", err);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": message}))
        }
    }
}

#[patch("/comments/{id}")]
async fn update_comment_handler(
    state: web::Data<ServiceRegister>,
    path: web::Path<uuid::Uuid>,
    payload: web::Json<UpdateCommentRequest>,
) -> impl Responder {
    let comment_id = path.into_inner();
    let result = state
        .comment_service
        .update_comment(
            comment_id,
            payload.id_post_comment,
            &payload.user_name_comment,
            &payload.comment,
        )
        .await;

    match result {
        Ok(comment) => {
            if let Some(comment) = comment {
                let json_response = serde_json::json!({
                    "status": "success",
                    "comment": comment
                });

                HttpResponse::Ok().json(json_response)
            } else {
                let message = format!("Comment with ID '{}' not found", comment_id);
                HttpResponse::NotFound().json(json!({"status": "fail", "message": message}))
            }
        }
        Err(err) => {
            let message = format!("Error while updating comment: {:?}", err);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": message}))
        }
    }
}

#[delete("/comments/{id}")]
async fn delete_comment_handler(
    state: web::Data<ServiceRegister>,
    path: web::Path<uuid::Uuid>,
) -> impl Responder {
    let comment_id = path.into_inner();
    let result = state.comment_service.delete_comment(comment_id).await;

    match result {
        Ok(_) => {
            let json_response = serde_json::json!({
                "status": "success",
                "message": "Comment deleted successfully"
            });

            HttpResponse::Ok().json(json_response)
        }
        Err(err) => {
            let message = format!("Error while deleting comment: {:?}", err);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": message}))
        }
    }
}
