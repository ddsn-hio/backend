#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::controller::middleware;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::_entities::events::{ActiveModel, Entity, Model};
use crate::models::_entities::users;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub start: DateTimeWithTimeZone,
    pub end: DateTimeWithTimeZone,
    pub lat: Decimal,
    pub long: Decimal,
    pub title: String,
    pub contents: String,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.start = Set(self.start.clone());
        item.end = Set(self.end.clone());
        item.lat = Set(self.lat.clone());
        item.long = Set(self.long.clone());
        item.title = Set(self.title.clone());
        item.contents = Set(self.contents.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    format::json(Entity::find().all(&ctx.db).await?)
}

#[debug_handler]
pub async fn add(
    auth: middleware::auth::ApiToken<users::Model>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    item.user_id = Set(auth.user.id);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn update(
    auth: middleware::auth::ApiToken<users::Model>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    item.user_id = Set(auth.user.id);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(
    auth: middleware::auth::ApiToken<users::Model>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    if item.user_id == auth.user.id {
        item.delete(&ctx.db).await?;
    }
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/events/")
        .add("/", get(list))
        .add("/", post(add))
        .add(":id", get(get_one))
        .add(":id", delete(remove))
        .add(":id", put(update))
}
