# League Stats

## Run locally

Requirements: Rust with Cargo and Docker Compose.

1. Start PostgreSQL: `docker compose up -d db`
2. Check `.env` and set `DATABASE_URL` if your database settings differ.
3. Start the app: `cargo run`
4. Open <http://127.0.0.1:3000>.

The app applies `migrations/0001_init.sql` at startup. The page can list accounts,
accept account records, and show combined stats for participants already in the
database. Riot API syncing is not implemented yet; accounts currently need a PUUID
entered manually, and the stats table stays empty until match data is added.

The credentials in `.env` and `compose.yaml` are for local development only.