// Copyright 2023 newbee1905 - Kodo <beenewminh@outlook.com>
//
// SPDX-License-Identifier: BSD-3-Clause

mod error;
mod utils;
mod handlers;

#[macro_use]
extern crate dotenv_codegen;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use bytes::Bytes;
use http_body_util::{combinators::BoxBody, BodyExt};
use hyper::{
	server::conn::http1,
	service::service_fn,
	body::Incoming,
	StatusCode,
	Method,
	Request,
	Response,
};
use tokio::net::TcpListener;
use hyper_util::rt::TokioIo;
use lazy_static::lazy_static;
use std::error::Error;
use sqlx::SqlitePool;
use urlencoding::decode;

use cs_handbook::db::get_db_pool;
use crate::handlers::{get_units_by_major};
use crate::utils::{full, create_error_response};

lazy_static! {
	static ref DB_URL: &'static str = dotenv!("DB_URL");
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
	pretty_env_logger::init();

	let pool = get_db_pool(*DB_URL).await?;
	let pool = Arc::new(pool);

	let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();

	let listener = TcpListener::bind(addr).await?;
	println!("Listening on http://{}", addr);
	loop {
		let (tcp, _) = listener.accept().await?;
		let io = TokioIo::new(tcp);
		let pool = pool.clone();

		tokio::task::spawn(async move {
			if let Err(err) = http1::Builder::new()
				.serve_connection(io, service_fn(move |req| router(req, pool.clone())))
				.await {
				println!("Error serving connection: {:?}", err);
			}
		});
	}
}

async fn router(req: Request<Incoming>, pool: Arc<SqlitePool>) -> Result<Response<BoxBody<Bytes, Infallible>>, hyper::Error> {
	let path_segments: Vec<&str> = req.uri().path().split('/').collect();

	let response = match (req.method(), path_segments.as_slice()) {
		(&Method::GET, ["", "major", major_title]) => {
			println!("{}", major_title);
			let major_title = decode(major_title).expect("UTF-8");
			println!("{}", major_title);
			let major_title = Arc::new(major_title.to_string());
			get_units_by_major(req, pool, major_title)
				.await
				.map_err(|e| create_error_response(&e))
		},
		_ => Ok(Response::builder()
				.status(StatusCode::NOT_FOUND)
				.body(full("Not Found"))
				.unwrap()),
	};

	response.or_else(|service_error_response| Ok(service_error_response))
}
