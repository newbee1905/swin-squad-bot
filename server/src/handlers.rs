// Copyright 2023 newbee1905 - Kodo <beenewminh@outlook.com>
//
// SPDX-License-Identifier: BSD-3-Clause

use crate::utils::full;
use crate::error::ServiceError;
use cs_handbook::db;
use std::collections::HashMap;
use sqlx::SqlitePool;
use std::convert::Infallible;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use std::sync::Arc;

use hyper::{
	header,
	StatusCode,
	Response,
	Request,
	body::Incoming,
};

pub async fn get_units_by_major(
	req: Request<Incoming>, 
	pool: Arc<SqlitePool>,
	major_title: Arc<String>,
) -> Result<Response<BoxBody<Bytes, Infallible>>, ServiceError> {
	println!("Major {}", major_title);

	let units = db::get_units_by_major(&pool, &major_title).await?;
	let json = serde_json::to_string(&units)?;

	Ok(Response::builder()
		.status(StatusCode::OK)
		.header(header::CONTENT_TYPE, "application/json")
		.body(full(json))
		.unwrap())
}
