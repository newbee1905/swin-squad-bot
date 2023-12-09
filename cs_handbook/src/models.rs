// Copyright 2023 newbee1905 - Kodo <beenewminh@outlook.com>
//
// SPDX-License-Identifier: BSD-3-Clause

pub type Unit = String;

pub struct Major {
	pub title: String,
	pub units: Vec<Unit>,  
}

pub struct Handbook {
	pub majors: Vec<Major>,
	pub cores: Vec<Unit>,
	pub electives: Vec<Unit>,
}

