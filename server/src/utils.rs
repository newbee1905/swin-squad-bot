// Copyright 2023 newbee1905 - Kodo <beenewminh@outlook.com>
//
// SPDX-License-Identifier: BSD-3-Clause

use std::collections::HashMap;
use http_body_util::{Full, BodyExt, combinators::BoxBody};
use hyper::{
	StatusCode, 
	Response,
	body::Body,
};
use std::convert::Infallible;
use bytes::Bytes;
use log::error;

use crate::error::ServiceError;

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, Infallible> {
	Full::new(chunk.into()).boxed()
}

pub fn create_error_response(e: &ServiceError) -> Response<BoxBody<Bytes, Infallible>> {
	let (status, message) = match e {
		ServiceError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
		ServiceError::DatabaseError(_) | ServiceError::SerializationError(_) => {
			(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
		},
	};
	error!("Error occurred: {}", e);
	Response::builder()
		.status(status)
		.body(full(message))
		.unwrap()
}

