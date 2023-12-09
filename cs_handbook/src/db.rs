// Copyright 2023 newbee1905 - Kodo <beenewminh@outlook.com>
//
// SPDX-License-Identifier: BSD-3-Clause

use crate::models::{Handbook, Unit};
use lazy_static::lazy_static;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool, Row};

lazy_static! {
	static ref SQL_MAJOR_TABLE: &'static str =
		"CREATE TABLE IF NOT EXISTS majors (
				title TEXT PRIMARY KEY NOT NULL
		)";
	static ref SQL_UNIT_TABLE: &'static str = 
		"CREATE TABLE IF NOT EXISTS units (
			name TEXT PRIMARY KEY NOT NULL,
			type TEXT NOT NULL, -- 'major', 'core', or 'elective'
			major_title TEXT, -- Can be NULL
			FOREIGN KEY (major_title)
			REFERENCES majors(title)
		)";
	static ref SQL_INSERT_MAJOR: &'static str =
		"INSERT INTO majors (title) VALUES (?) ON CONFLICT(title) DO NOTHING";

	static ref SQL_INSERT_UPDATE_UNIT: &'static str =
		"INSERT INTO units (name, type, major_title) VALUES (?, ?, ?) \
		 ON CONFLICT(name) DO UPDATE SET major_title = excluded.major_title, type = excluded.type";

	static ref SQL_DELETE_UNIT: &'static str =
		"DELETE FROM units WHERE major_title = ? AND type = ? AND name NOT IN (";

	static ref SQL_GET_UNIT_BY_MAJOR: &'static str =
		"SELECT name FROM units WHERE major_title = ?";

	static ref SQL_GET_UNIT_BY_CODENAME: &'static str =
		"SELECT name FROM units WHERE name = ?";
}

pub async fn get_db_pool(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
	if !Sqlite::database_exists(db_url).await? {
		match Sqlite::create_database(db_url).await {
			Ok(_) => println!("Create db success"),
			Err(error) => panic!("error: {}", error),
		}
	}

	let pool = SqlitePool::connect(db_url).await?;

	Ok(pool)
}

pub async fn create_tables_if_not_exists(pool: &SqlitePool) -> Result<(), sqlx::Error> {
	sqlx::query(*SQL_MAJOR_TABLE).execute(pool).await?;
	sqlx::query(*SQL_UNIT_TABLE).execute(pool).await?;

	Ok(())
}

pub async fn update_handbook(pool: &SqlitePool, handbook: &Handbook) -> Result<(), sqlx::Error> {
	let mut transaction = pool.begin().await?;

	for major in &handbook.majors {
		sqlx::query(*SQL_INSERT_MAJOR)
			.bind(&major.title)
			.execute(&mut transaction)
			.await?;

		update_units(&mut transaction, Some(&major.title), "major", &major.units).await?;
	}

	update_units(&mut transaction, None, "core", &handbook.cores).await?;
	update_units(&mut transaction, None, "elective", &handbook.electives).await?;

	transaction.commit().await?;

	Ok(())
}

async fn update_units(
	transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
	major_title: Option<&str>,
	unit_type: &str,
	units: &Vec<String>
) -> Result<(), sqlx::Error> {
	let mut unit_names = Vec::new();
	unit_names.reserve(units.len());

	for unit in units {
		sqlx::query(*SQL_INSERT_UPDATE_UNIT)
			.bind(unit)
			.bind(unit_type)
			.bind(major_title)
			.execute(&mut *transaction)
			.await?;

		unit_names.push(format!("'{}'", unit));
	}

	if !units.is_empty() && unit_type != "elective" {
		let delete_query = format!("{}{})", *SQL_DELETE_UNIT, unit_names.join(","));
		sqlx::query(&delete_query)
			.bind(major_title)
			.bind(unit_type)
			.execute(&mut *transaction)
			.await?;
	}

	Ok(())
}

pub async fn get_units_by_major(pool: &SqlitePool, major_title: &str) -> Result<Vec<Unit>, sqlx::Error> {
	Ok(
		sqlx::query(*SQL_GET_UNIT_BY_MAJOR)
			.bind(major_title)
			.map(|row| row.get::<Unit, _>("name"))
			.fetch_all(pool)
			.await?
	)
}

pub async fn get_units_by_codename(pool: &SqlitePool, codename: &str) -> Result<Unit, sqlx::Error> {
	Ok(
		sqlx::query(*SQL_GET_UNIT_BY_CODENAME)
			.bind(codename)
			.map(|row| row.get::<Unit, _>("name"))
			.fetch_one(pool)
			.await?
	)
}
