use axum::{Extension, Json};
use entity::filling;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Filling {
    name: String,
}

#[derive(Serialize)]
pub struct ResFilling {
    id: i32,
    name: String,
}

pub async fn filling_save(
    Extension(db): Extension<DatabaseConnection>,
    Json(new_filling): Json<Filling>,
) -> Json<ResFilling> {
    let filling = filling::ActiveModel {
        name: Set(new_filling.name),
        ..Default::default()
    };

    let res = filling.save(&db).await.unwrap();
    Json(ResFilling {
        id: res.id.unwrap(),
        name: res.name.unwrap(),
    })
}
