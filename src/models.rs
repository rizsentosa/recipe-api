use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::Json};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Recipe {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub ingredients: Json<Vec<Ingredient>>,
    pub instructions: String,
    pub prep_time_minutes: i32,
    pub cook_time_minutes: i32,
    pub servings: i32,
    pub is_favorite: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RecipeInput {
    pub title: String,
    pub description: Option<String>,
    pub ingredients: Json<Vec<Ingredient>>,
    pub instructions: String,
    #[serde(default)]
    pub prep_time_minutes: i32,
    #[serde(default)]
    pub cook_time_minutes: i32,
    #[serde(default = "default_servings")]
    pub servings: i32,
}

fn default_servings() -> i32 {
    1
}

impl RecipeInput {
    pub fn validate(&self) -> Result<(), AppError> {
        let fail = |m: &str| Err(AppError::Validation(m.into()));

        if self.title.trim().is_empty() {
            return fail("title must not be empty");
        }
        if self.instructions.trim().is_empty() {
            return fail("instructions must not be empty");
        }
        if self.ingredients.is_empty() {
            return fail("at least one ingredient is required");
        }
        if self.ingredients.iter().any(|i| i.name.trim().is_empty()) {
            return fail("ingredient names must not be empty");
        }
        if self.prep_time_minutes < 0 || self.cook_time_minutes < 0 {
            return fail("times must not be negative");
        }
        if self.servings < 1 {
            return fail("servings must be at least 1");
        }
        Ok(())
    }
}
