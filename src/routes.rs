use axum::{Router, routing::get, routing::put};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

use crate::handlers::*;

pub fn router(pool: PgPool) -> Router {
    let api = Router::new()
        .route("/recipes", get(list_recipes).post(create_recipe))
        .route(
            "/recipes/{id}",
            get(get_recipe).put(update_recipe).delete(delete_recipe),
        )
        .route(
            "/recipes/{id}/favorite",
            put(add_favorite).delete(remove_favorite),
        )
        .route("/favorites", get(list_favorites));

    Router::new()
        .nest("/api/v1", api)
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
}
