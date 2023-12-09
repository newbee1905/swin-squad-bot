// Copyright 2023 newbee1905 - Kodo <beenewminh@outlook.com>
//
// SPDX-License-Identifier: BSD-3-Clause

use std::error::Error;

use crate::models::{Handbook, Major};
use scraper::{Selector, ElementRef, html::Html};
use lazy_static::lazy_static;

lazy_static! {
	static ref COURSEBOOK_URL: &'static str
		= "https://www.swinburne.edu.au/course/undergraduate/bachelor-of-computer-science/handbook";

	static ref STUDY_STRUCTURE_SELECTOR: &'static str = "#study-structure";
	static ref CORE_UNITS_LIST_SELECTOR: &'static str
		= "#contentblock_YP36DOOFP td:nth-child(2)";
	static ref MAJORS_LIST_SELECTOR: &'static str
		= "#contentblock_copy_TD1TDRB0J_accordion_body section.unit-table";
	static ref MAJOR_TITLE_SELECTOR: &'static str = "h4";
	static ref MAJOR_UNITS_SELECTOR: &'static str
		= "td:nth-child(2)";
	static ref ELECTIVE_UNITS_LIST_SELECTOR: &'static str
		= "#text_1588112631_RIUDS7CWR li";
}

pub async fn parse_handbook() -> Result<Handbook, Box<dyn Error>> {
	let res = reqwest::get(*COURSEBOOK_URL).await?;

	if !res.status().is_success() {
		return Err(format!("Failed to fetch data: {}", res.status()).into());
	}

	let body = res.text().await?;

	let document = Html::parse_document(&body);

	let study_structure = parse_study_structure(&document)?;

	let majors = parse_majors(&study_structure)?;
	let cores = parse_cores(&study_structure)?;
	let electives = parse_electives(&study_structure)?;

	Ok(Handbook {
		majors,
		cores,
		electives,
	})
}

fn parse_study_structure<'a>(document: &'a Html) -> Result<ElementRef<'a>, Box<dyn Error>> {
	let study_structure_selector = Selector::parse(*STUDY_STRUCTURE_SELECTOR)?;

	Ok(
		document
			.select(&study_structure_selector)
			.next()
			.expect("Failed to get Study Structure Element")
	)
}

fn parse_majors<'a>(study_structure: &ElementRef<'a>) -> Result<Vec<Major>, Box<dyn Error>> {
	let majors_list_selector = Selector::parse(*MAJORS_LIST_SELECTOR)?;
	let major_title_selector = Selector::parse(*MAJOR_TITLE_SELECTOR)?;
	let major_units_selector = Selector::parse(*MAJOR_UNITS_SELECTOR)?;

	Ok(
		study_structure
			.select(&majors_list_selector)
			.map(|block| {
				let title = block
					.select(&major_title_selector)
					.next()
					.expect("Failed getting title")
					.text()
					.map(|s| s.trim())
					.collect();

				let units = block
					.select(&major_units_selector)
					.map(|unit| {
						unit
							.text()
							.map(|s| s.trim())
							.collect()
					})
					.collect::<Vec<String>>();

				Major { title, units }
			})
			.collect::<Vec<Major>>()
	)
}

fn parse_cores<'a>(study_structure: &ElementRef<'a>) -> Result<Vec<String>, Box<dyn Error>> {
	let core_units_selector = Selector::parse(*CORE_UNITS_LIST_SELECTOR)?;

	Ok(
		study_structure
			.select(&core_units_selector)
			.map(|unit| {
				unit
					.text()
					.map(|s| s.trim())
					.collect()
			})
			.collect::<Vec<String>>()
	)
}

fn parse_electives<'a>(study_structure: &ElementRef<'a>) -> Result<Vec<String>, Box<dyn Error>> {
	let elective_units_selector = Selector::parse(*ELECTIVE_UNITS_LIST_SELECTOR)?;

	Ok(
		study_structure
			.select(&elective_units_selector)
			.map(|unit| {
				unit
					.text()
					// using unwrap cause I am getting first element
					// which will always be not a None
					.map(|s| s.trim().split_whitespace().next().unwrap_or(""))
					.collect()
			})
			.collect::<Vec<String>>()
	)
}

