use sqlx::PgPool;

#[derive(sqlx::FromRow)]
pub struct CombinedStats {
	pub games: i64,
	pub wins: i64,
	pub kills: i64,
	pub deaths: i64,
	pub assists: i64,
}

pub async fn combined(pool: &PgPool) -> Result<CombinedStats, sqlx::Error> {
	sqlx::query_as::<_, CombinedStats>(
		"SELECT COUNT(*) AS games, \
		 COUNT(*) FILTER (WHERE p.win) AS wins, \
		 COALESCE(SUM(p.kills), 0) AS kills, \
		 COALESCE(SUM(p.deaths), 0) AS deaths, \
		 COALESCE(SUM(p.assists), 0) AS assists \
		 FROM participants p \
		 INNER JOIN accounts a ON a.puuid = p.puuid",
	)
	.fetch_one(pool)
	.await
}
