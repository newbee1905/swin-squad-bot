// Copyright 2023 newbee1905 - Kodo <beenewminh@outlook.com>
//
// SPDX-License-Identifier: BSD-3-Clause

#[macro_use]
extern crate dotenv_codegen;

mod models;
mod db;
mod scraper;

use crate::scraper::parse_handbook;
use crate::db::{create_tables_if_not_exists, update_handbook};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::error::Error;
use lazy_static::lazy_static;

lazy_static! {
	static ref DB_URL: &'static str = dotenv!("DB_URL");
}

/// Entry point of the application.
/// - Fetches the handbook data by scraping the web.
/// - Create the SQLite database if it does not exist.
/// - Connects to the SQLite database.
/// - Creates necessary tables if they do not exist.
/// - Updates the database with the fetched handbook data.
/// - Closes the database connection after updating.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let handbook = parse_handbook().await?;

	if !Sqlite::database_exists(*DB_URL).await? {
		match Sqlite::create_database(*DB_URL).await {
			Ok(_) => println!("Create db success"),
			Err(error) => panic!("error: {}", error),
		}
	}

	let pool = SqlitePool::connect(*DB_URL).await?;
	create_tables_if_not_exists(&pool).await?;
	update_handbook(&pool, &handbook).await?;
	pool.close().await;

	Ok(())
}
