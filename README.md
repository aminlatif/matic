# matic
Simple and lightweight organizer based on Rust's Axum.

### Crates to consider:
```
# Auth & Security
axum-login = "0.4"
jsonwebtoken = "9.2"
argon2 = "0.5"
cookie = "0.18"
rand = "0.8"

# Validation & Serialization
validator = { version = "0.16", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Configuration
config = "0.13"
dotenvy = "0.15"

# Error Handling
thiserror = "1.0"
anyhow = "1.0"

# Rate Limiting
tower-governor = "0.3"

# Caching (Redis)
redis = { version = "0.23", features = ["tokio-comp"] }

# Email
lettre = "0.11"

# File Uploads
multer = "3.0"

# Testing
testcontainers = { version = "0.16", features = ["postgres"] }
```

### sqlx create migrations
* Create migration: `sqlx migrate add <name>`
* Write SQL in the generated `.sql` file
* Run migrations: `sqlx migrate run`
* For runtime: Use `sqlx::migrate!().run(pool).await` in your code
* Ensure `DATABASE_URL` is set in .env or environment.
