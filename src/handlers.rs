use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::CurrentUser,
    error::AppError,
    models::{Recipe, RecipeInput},
    pagination::{ListParams, Page},
    repo,
};

type ApiResult<T> = Result<T, AppError>;

pub async fn list_recipes(
    State(pool): State<PgPool>,
    CurrentUser(user): CurrentUser,
    Query(params): Query<ListParams>,
) -> ApiResult<Json<Page<Recipe>>> {
    Ok(Json(repo::list(&pool, user, &params, false).await?))
}

pub async fn get_recipe(
    State(pool): State<PgPool>,
    CurrentUser(user): CurrentUser,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Recipe>> {
    Ok(Json(repo::get(&pool, user, id).await?))
}

pub async fn create_recipe(
    State(pool): State<PgPool>,
    CurrentUser(user): CurrentUser,
    Json(input): Json<RecipeInput>,
) -> ApiResult<(StatusCode, Json<Recipe>)> {
    input.validate()?;
    let recipe = repo::create(&pool, user, &input).await?;
    Ok((StatusCode::CREATED, Json(recipe)))
}

pub async fn update_recipe(
    State(pool): State<PgPool>,
    CurrentUser(user): CurrentUser,
    Path(id): Path<Uuid>,
    Json(input): Json<RecipeInput>,
) -> ApiResult<Json<Recipe>> {
    input.validate()?;
    Ok(Json(repo::update(&pool, user, id, &input).await?))
}

pub async fn delete_recipe(
    State(pool): State<PgPool>,
    _user: CurrentUser,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    repo::delete(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_favorites(
    State(pool): State<PgPool>,
    CurrentUser(user): CurrentUser,
    Query(params): Query<ListParams>,
) -> ApiResult<Json<Page<Recipe>>> {
    Ok(Json(repo::list(&pool, user, &params, true).await?))
}

pub async fn add_favorite(
    State(pool): State<PgPool>,
    CurrentUser(user): CurrentUser,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    repo::add_favorite(&pool, user, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn remove_favorite(
    State(pool): State<PgPool>,
    CurrentUser(user): CurrentUser,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    repo::remove_favorite(&pool, user, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
