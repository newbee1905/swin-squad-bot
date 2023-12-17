// Copyright 2023 newbee1905 - Kodo <beenewminh@outlook.com>
//
// SPDX-License-Identifier: BSD-3-Clause

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ServiceError {
	BadRequest(String),
	DatabaseError(sqlx::Error),
	SerializationError(serde_json::Error),
}

impl fmt::Display for ServiceError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ServiceError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
			ServiceError::DatabaseError(e) => write!(f, "Database error: {}", e),
			ServiceError::SerializationError(e) => write!(f, "Serialization error: {}", e),
		}
	}
}

impl Error for ServiceError {}

impl From<sqlx::Error> for ServiceError {
	fn from(e: sqlx::Error) -> Self {
		ServiceError::DatabaseError(e)
	}
}

impl From<serde_json::Error> for ServiceError {
	fn from(e: serde_json::Error) -> Self {
		ServiceError::SerializationError(e)
	}
}
