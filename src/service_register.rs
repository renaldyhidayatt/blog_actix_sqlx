use std::sync::Arc;

use crate::{
    abstract_trait::{
        DynCategoryRepository, DynCategoryService, DynCommentRepository, DynCommentsService,
        DynPostsRepository, DynPostsService, DynUserRepository, DynUserService,
    },
    config::{Config, ConnectionPool},
    repository::{CategoryRepository, CommentRepository, PostsRepository, UserRepository},
    service::{CategoryService, CommentService, PostsService, UserService},
};

#[derive(Clone)]
pub struct ServiceRegister {
    pub env: Config,
    pub category_service: DynCategoryService,
    pub user_service: DynUserService,
    pub posts_service: DynPostsService,
    pub comment_service: DynCommentsService,
}

impl ServiceRegister {
    pub fn new(pool: ConnectionPool, config: Config) -> Self {
        let category_repository =
            Arc::new(CategoryRepository::new(pool.clone())) as DynCategoryRepository;

        let category_service =
            Arc::new(CategoryService::new(category_repository)) as DynCategoryService;

        let user_repository = Arc::new(UserRepository::new(pool.clone())) as DynUserRepository;
        let user_service = Arc::new(UserService::new(user_repository)) as DynUserService;

        let posts_repository = Arc::new(PostsRepository::new(pool.clone())) as DynPostsRepository;
        let posts_service = Arc::new(PostsService::new(posts_repository)) as DynPostsService;

        let comment_repository =
            Arc::new(CommentRepository::new(pool.clone())) as DynCommentRepository;
        let comment_service =
            Arc::new(CommentService::new(comment_repository)) as DynCommentsService;

        ServiceRegister {
            env: config.clone(),
            category_service,
            user_service,
            posts_service,
            comment_service,
        }
    }
}
