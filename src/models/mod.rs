use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Dog {
    pub id: i32,
    pub name: String,
    pub image: String,
}
