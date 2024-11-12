use axum::{Extension, Json};
use entity::cake;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Cake {
    name: String,
}

#[derive(Serialize)]
pub struct ResCake {
    id: i32,
    name: String,
}

pub async fn cake_save(
    Extension(db): Extension<DatabaseConnection>,
    Json(new_cake): Json<Cake>,
) -> Json<ResCake> {
    let cake = cake::ActiveModel {
        name: Set(new_cake.name.to_owned()),
        ..Default::default()
    };

    let res = cake.save(&db).await.unwrap();
    Json(ResCake {
        id: res.id.unwrap(),
        name: res.name.unwrap(),
    })
}
