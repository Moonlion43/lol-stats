use askama::Template;
use axum::{extract::State, Form, response::{Html, Redirect}};
use serde::Deserialize;

use crate::{
	db::{accounts::{self, Account}, stats::{self, CombinedStats}},
	error::AppError,
};

use super::state::AppState;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
	accounts: Vec<Account>,
	stats: CombinedStats,
}

pub async fn index(State(state): State<AppState>) -> Result<Html<String>, AppError> {
	let template = IndexTemplate {
		accounts: accounts::list(&state.pool).await?,
		stats: stats::combined(&state.pool).await?,
	};
	Ok(Html(template.render()?))
}

#[derive(Deserialize)]
pub struct NewAccount {
	puuid: String,
	game_name: String,
	tag_line: String,
	region: String,
}

pub async fn add_account(
	State(state): State<AppState>,
	Form(form): Form<NewAccount>,
) -> Result<Redirect, AppError> {
	let account = Account {
		puuid: form.puuid.trim().to_owned(),
		game_name: form.game_name.trim().to_owned(),
		tag_line: form.tag_line.trim().to_owned(),
		region: form.region.trim().to_owned(),
	};

	if [&account.puuid, &account.game_name, &account.tag_line, &account.region]
		.iter()
		.any(|value| value.is_empty())
	{
		return Err(AppError::BadRequest("All account fields are required"));
	}

	accounts::insert(&state.pool, &account).await?;
	Ok(Redirect::to("/"))
}
