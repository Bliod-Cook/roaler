use axum::{Json, Router, extract::{Path, State}, routing::{get, put}};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use roaler_domain::{
    AppContext, AppResult,
    collections,
    models::{
        collection::{Collection, CollectionSummary, CreateCollectionInput, UpdateCollectionInput},
        system::ApiMessage,
    },
};

use crate::authn::require_admin;

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/", get(list_collections).post(create_collection))
        .route("/{collection_id}", put(update_collection).delete(delete_collection))
}

async fn list_collections(
    State(context): State<AppContext>,
    jar: CookieJar,
) -> AppResult<Json<Vec<CollectionSummary>>> {
    require_admin(&context, &jar).await?;
    Ok(Json(collections::list_collections(&context).await?))
}

async fn create_collection(
    State(context): State<AppContext>,
    jar: CookieJar,
    Json(input): Json<CreateCollectionInput>,
) -> AppResult<Json<Collection>> {
    require_admin(&context, &jar).await?;
    Ok(Json(collections::create_collection(&context, &input).await?))
}

async fn update_collection(
    State(context): State<AppContext>,
    jar: CookieJar,
    Path(collection_id): Path<Uuid>,
    Json(input): Json<UpdateCollectionInput>,
) -> AppResult<Json<Collection>> {
    require_admin(&context, &jar).await?;
    Ok(Json(collections::update_collection(&context, collection_id, &input).await?))
}

async fn delete_collection(
    State(context): State<AppContext>,
    jar: CookieJar,
    Path(collection_id): Path<Uuid>,
) -> AppResult<Json<ApiMessage>> {
    require_admin(&context, &jar).await?;
    collections::delete_collection(&context, collection_id).await?;
    Ok(Json(ApiMessage {
        message: "collection deleted".to_owned(),
    }))
}
