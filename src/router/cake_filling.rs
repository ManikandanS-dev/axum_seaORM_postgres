use axum::{Extension, Json};
use entity::cake_filling;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CakeFilling {
    cake_id: i32,
    filling_id: i32,
}

#[derive(Serialize)]
pub struct ResCakeFilling {
    id: i32,
    cake_id: i32,
    filling_id: i32,
}

pub async fn cake_filling_save(
    Extension(db): Extension<DatabaseConnection>,
    Json(new_cake_filling): Json<CakeFilling>,
) -> Json<ResCakeFilling> {
    let cake_filling = cake_filling::ActiveModel {
        cake_id: Set(new_cake_filling.cake_id),
        filling_id: Set(new_cake_filling.filling_id),
        ..Default::default()
    };

    let res = cake_filling.save(&db).await.unwrap();
    Json(ResCakeFilling {
        id: res.id.unwrap(),
        cake_id: res.cake_id.unwrap(),
        filling_id: res.filling_id.unwrap(),
    })
}
