use axum::{Extension, Json};
use entity::fruit;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Fruit {
    name: String,
    cake_id: i32,
}

#[derive(Serialize)]
pub struct ResFruit {
    id: i32,
    name: String,
    cake_id: i32,
}

pub async fn fruit_save(
    Extension(db): Extension<DatabaseConnection>,
    Json(new_fruit): Json<Fruit>,
) -> Json<ResFruit> {
    let fruit = fruit::ActiveModel {
        name: Set(new_fruit.name),
        cake_id: Set(new_fruit.cake_id),
        ..Default::default()
    };

    let res = fruit.save(&db).await.unwrap();
    Json(ResFruit {
        id: res.id.unwrap(),
        name: res.name.unwrap(),
        cake_id: res.cake_id.unwrap(),
    })
}
