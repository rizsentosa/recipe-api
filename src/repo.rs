use sqlx::{PgPool, Postgres, QueryBuilder, types::Json};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{Recipe, RecipeInput},
    pagination::{ListParams, Page},
};

/// The single SELECT used by every read path (get, list, favorites).
/// Ends with `WHERE TRUE` so callers can append `AND ...` conditions.
fn select_recipes(user: Uuid) -> QueryBuilder<'static, Postgres> {
    let mut qb = QueryBuilder::new(
        "SELECT r.id, r.title, r.description, r.ingredients, r.instructions, \
                r.prep_time_minutes, r.cook_time_minutes, r.servings, \
                r.created_at, r.updated_at, \
                EXISTS (SELECT 1 FROM favorites f \
                        WHERE f.recipe_id = r.id AND f.user_id = ",
    );
    qb.push_bind(user)
        .push(") AS is_favorite FROM recipes r WHERE TRUE");
    qb
}

/// Filters shared by the data query and the COUNT query.
fn apply_filters(
    qb: &mut QueryBuilder<'_, Postgres>,
    user: Uuid,
    params: &ListParams,
    favorites_only: bool,
) {
    if favorites_only {
        qb.push(
            " AND EXISTS (SELECT 1 FROM favorites fav \
               WHERE fav.recipe_id = r.id AND fav.user_id = ",
        )
        .push_bind(user)
        .push(")");
    }
    if let Some(q) = params.q.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        let pattern = format!("%{q}%");
        qb.push(" AND (r.title ILIKE ")
            .push_bind(pattern.clone())
            .push(" OR r.description ILIKE ")
            .push_bind(pattern)
            .push(")");
    }
}

pub async fn get(pool: &PgPool, user: Uuid, id: Uuid) -> Result<Recipe, AppError> {
    let mut qb = select_recipes(user);
    qb.push(" AND r.id = ").push_bind(id);
    qb.build_query_as::<Recipe>()
        .fetch_optional(pool)
        .await?
        .ok_or(AppError::NotFound)
}

/// One function powers both `/recipes` and `/favorites`.
pub async fn list(
    pool: &PgPool,
    user: Uuid,
    params: &ListParams,
    favorites_only: bool,
) -> Result<Page<Recipe>, AppError> {
    let mut count = QueryBuilder::new("SELECT COUNT(*) FROM recipes r WHERE TRUE");
    apply_filters(&mut count, user, params, favorites_only);
    let total: i64 = count.build_query_scalar().fetch_one(pool).await?;

    let mut qb = select_recipes(user);
    apply_filters(&mut qb, user, params, favorites_only);
    qb.push(" ORDER BY r.created_at DESC, r.id LIMIT ")
        .push_bind(params.per_page())
        .push(" OFFSET ")
        .push_bind(params.offset());
    let items = qb.build_query_as::<Recipe>().fetch_all(pool).await?;

    Ok(Page::new(items, params, total))
}

pub async fn create(pool: &PgPool, user: Uuid, input: &RecipeInput) -> Result<Recipe, AppError> {
    let id: Uuid = sqlx::query_scalar(
        "INSERT INTO recipes
            (title, description, ingredients, instructions,
             prep_time_minutes, cook_time_minutes, servings)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING id",
    )
    .bind(&input.title)
    .bind(&input.description)
    .bind(Json(&input.ingredients))
    .bind(&input.instructions)
    .bind(input.prep_time_minutes)
    .bind(input.cook_time_minutes)
    .bind(input.servings)
    .fetch_one(pool)
    .await?;
    get(pool, user, id).await
}

pub async fn update(
    pool: &PgPool,
    user: Uuid,
    id: Uuid,
    input: &RecipeInput,
) -> Result<Recipe, AppError> {
    let updated: Option<Uuid> = sqlx::query_scalar(
        "UPDATE recipes SET
            title = $2, description = $3, ingredients = $4, instructions = $5,
            prep_time_minutes = $6, cook_time_minutes = $7, servings = $8,
            updated_at = now()
         WHERE id = $1
         RETURNING id",
    )
    .bind(id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(Json(&input.ingredients))
    .bind(&input.instructions)
    .bind(input.prep_time_minutes)
    .bind(input.cook_time_minutes)
    .bind(input.servings)
    .fetch_optional(pool)
    .await?;

    updated.ok_or(AppError::NotFound)?;
    get(pool, user, id).await
}

pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let res = sqlx::query("DELETE FROM recipes WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(())
}

/// Idempotent: favoriting twice is fine. A missing recipe surfaces as a
/// foreign-key violation, which `AppError` maps to 404.
pub async fn add_favorite(pool: &PgPool, user: Uuid, recipe: Uuid) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO favorites (user_id, recipe_id) VALUES ($1, $2)
         ON CONFLICT DO NOTHING",
    )
    .bind(user)
    .bind(recipe)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove_favorite(pool: &PgPool, user: Uuid, recipe: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM favorites WHERE user_id = $1 AND recipe_id = $2")
        .bind(user)
        .bind(recipe)
        .execute(pool)
        .await?;
    Ok(())
}
