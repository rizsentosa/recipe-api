-- Add migration script here
-- Add migration script here
CREATE TABLE recipes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    title TEXT NOT NULL,
    description TEXT,
    ingredients JSONB NOT NULL DEFAULT '[]',
    instructions TEXT NOT NULL,
    prep_time_minutes INT NOT NULL DEFAULT 0 CHECK (prep_time_minutes >= 0),
    cook_time_minutes INT NOT NULL DEFAULT 0 CHECK (cook_time_minutes >= 0),
    servings INT NOT NULL DEFAULT 1 CHECK (servings > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_recipes_created_at ON recipes (created_at DESC, id);

CREATE TABLE favorites (
    user_id UUID NOT NULL,
    recipe_id UUID NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, recipe_id)
);

CREATE INDEX idx_favorites_recipe ON favorites (recipe_id);