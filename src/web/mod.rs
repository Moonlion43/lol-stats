pub mod handlers;
pub mod state;

use axum::{routing::{get, post}, Router};
use tower_http::services::ServeDir;

pub fn router(state: state::AppState) -> Router {
	Router::new()
		.route("/", get(handlers::index))
		.route("/accounts", post(handlers::add_account))
		.nest_service("/static", ServeDir::new("static"))
		.with_state(state)
}
