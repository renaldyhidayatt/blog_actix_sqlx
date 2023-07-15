use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCategoryRequest {
    pub name: String,
}
