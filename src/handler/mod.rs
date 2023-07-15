use actix_web::web;
mod auth;
mod category;
mod comment;
mod hello;
mod posts;

use self::hello::health_checker_handler;

use self::auth::{get_me_handler, login_user_handler, logout_handler, register_user_handler};
use self::category::{
    create_category_handler, delete_category_handler, edit_category_handler,
    get_categories_handler, get_category_handler,
};

use self::comment::{
    create_comment_handler, delete_comment_handler, get_comment_handler, get_comments_handler,
    update_comment_handler,
};
use self::posts::{
    create_post_handler, delete_post_handler, get_post_handler, get_posts_handler,
    update_post_handler,
};

pub fn router_config(conf: &mut web::ServiceConfig) {
    let router = web::scope("/api")
        .service(health_checker_handler)
        .service(register_user_handler)
        .service(login_user_handler)
        .service(get_me_handler)
        .service(logout_handler)
        .service(get_categories_handler)
        .service(get_category_handler)
        .service(create_category_handler)
        .service(edit_category_handler)
        .service(delete_category_handler)
        .service(get_posts_handler)
        .service(get_post_handler)
        .service(create_post_handler)
        .service(update_post_handler)
        .service(delete_post_handler)
        .service(get_comments_handler)
        .service(get_comment_handler)
        .service(create_comment_handler)
        .service(update_comment_handler)
        .service(delete_comment_handler);

    conf.service(router);
}
