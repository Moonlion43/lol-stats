CREATE TABLE accounts (
    id BIGSERIAL PRIMARY KEY,
    puuid TEXT NOT NULL UNIQUE,
    game_name TEXT NOT NULL,
    tag_line TEXT NOT NULL,
    region TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE matches (
    match_id TEXT PRIMARY KEY,
    queue_id INTEGER,
    game_start TIMESTAMPTZ,
    game_duration_seconds INTEGER
);

CREATE TABLE participants (
    match_id TEXT NOT NULL REFERENCES matches(match_id) ON DELETE CASCADE,
    puuid TEXT NOT NULL,
    champion_id INTEGER NOT NULL,
    team_id INTEGER NOT NULL,
    win BOOLEAN NOT NULL,
    kills INTEGER NOT NULL DEFAULT 0,
    deaths INTEGER NOT NULL DEFAULT 0,
    assists INTEGER NOT NULL DEFAULT 0,
    total_damage_to_champions BIGINT NOT NULL DEFAULT 0,
    gold_earned INTEGER NOT NULL DEFAULT 0,
    cs INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (match_id, puuid)
);

CREATE INDEX participants_puuid_idx ON participants(puuid);
