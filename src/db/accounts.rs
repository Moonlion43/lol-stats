use sqlx::PgPool;

#[derive(sqlx::FromRow)]
pub struct Account {
	pub puuid: String,
	pub game_name: String,
	pub tag_line: String,
	pub region: String,
}

pub async fn list(pool: &PgPool) -> Result<Vec<Account>, sqlx::Error> {
	sqlx::query_as::<_, Account>(
		"SELECT puuid, game_name, tag_line, region FROM accounts ORDER BY game_name, tag_line",
	)
	.fetch_all(pool)
	.await
}

pub async fn insert(pool: &PgPool, account: &Account) -> Result<(), sqlx::Error> {
	sqlx::query(
		"INSERT INTO accounts (puuid, game_name, tag_line, region) \
		 VALUES ($1, $2, $3, $4) ON CONFLICT (puuid) DO NOTHING",
	)
	.bind(&account.puuid)
	.bind(&account.game_name)
	.bind(&account.tag_line)
	.bind(&account.region)
	.execute(pool)
	.await?;
	Ok(())
}
