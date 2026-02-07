# bananatree

Small personal playground project: me trying out Rust + Actix and wiring it up to Postgres.

It is a minimal text-post board API (backend only). There is no frontend in this repo.

## What It Does
- `POST /post_banana`: insert a text post (`user_id`, `content`)
- `GET /recent_bananas`: fetch the 10 most recent posts

Data is stored in Postgres tables `users` and `bananas` (created automatically on startup).

## Running
1. Install Postgres and start it.
2. Create a role + database (see `setupnotes.txt` for example SQL).
3. Update the Postgres connection string in `src/db.rs` (currently hardcoded).
4. Run the server:

```bash
cargo run
```

## Quick Manual Testing
```bash
./testpost.sh
./testrecent.sh
```
