mod category;
mod comment;
mod posts;
mod user;

pub use self::category::{
    CategoryRepositoryTrait, CategoryServiceTrait, DynCategoryRepository, DynCategoryService,
};

pub use self::comment::{
    CommentRepositoryTrait, CommentServiceTrait, DynCommentRepository, DynCommentService,
};
pub use self::posts::{
    DynPostsRepository, DynPostsService, PostsRepositoryTrait, PostsServiceTrait,
};
pub use self::user::{DynUserRepository, DynUserService, UserRepositoryTrait, UserServiceTrait};
