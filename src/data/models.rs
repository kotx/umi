use serde::Serialize;

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UmiUser {
    pub id: String,
    pub opt: bool,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct UmiMessage {
    pub id: String,
    pub author: String,
    pub content: String,
}
