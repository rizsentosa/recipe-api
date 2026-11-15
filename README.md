# Recipe API (Rust + Axum + SQLx + PostgreSQL)

## Run
```bash
cp .env.example .env
docker compose up -d        # PostgreSQL
cargo run                   # runs migrations on startup
```

## Endpoints (all under /api/v1, all need `X-User-Id: <uuid>`)
| Method | Path                    | Purpose                              |
|--------|-------------------------|--------------------------------------|
| POST   | /recipes                | create recipe                        |
| GET    | /recipes                | paginated list (`page`,`per_page`,`q`) |
| GET    | /recipes/{id}           | get one                              |
| PUT    | /recipes/{id}           | edit (full replace)                  |
| DELETE | /recipes/{id}           | delete                               |
| PUT    | /recipes/{id}/favorite  | add to my favorites (idempotent)     |
| DELETE | /recipes/{id}/favorite  | remove from my favorites             |
| GET    | /favorites              | my favorites, paginated              |

## Try it
```bash
U="X-User-Id: 11111111-1111-1111-1111-111111111111"

curl -X POST localhost:3080/api/v1/recipes -H "$U" -H 'Content-Type: application/json' -d '{
  "title": "Nasi Goreng",
  "description": "Indonesian fried rice",
  "ingredients": [{"name":"cooked rice","quantity":"3 cups"},{"name":"kecap manis","quantity":"2 tbsp"}],
  "instructions": "Stir-fry everything on high heat.",
  "prep_time_minutes": 10, "cook_time_minutes": 10, "servings": 2
}'

curl "localhost:3080/api/v1/recipes?page=1&per_page=10&q=nasi" -H "$U"
curl -X PUT localhost:3080/api/v1/recipes/<id>/favorite -H "$U"
curl "localhost:3080/api/v1/favorites" -H "$U"
```
