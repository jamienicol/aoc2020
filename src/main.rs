use anyhow::Result;
use itertools::Itertools;
use nom::{
    IResult,
    character::complete::{alpha1, char, digit1, space1},
    combinator::map_res,
};
use thiserror::Error;

use std::ops::RangeInclusive;

#[derive(Error, Debug)]
enum AocError {
    #[error("No entries exist with a matching sum")]
    BadEntries,
    #[error("Parsing passwords file failed")]
    PasswordParsingError,
}

fn find_product_of_entries_with_sum(entries: &[i32], num_entries: usize, sum: i32) -> Result<i32> {
    for combination in entries.iter().combinations(num_entries) {
        if combination.iter().cloned().sum::<i32>() == sum {
            return Ok(combination.iter().cloned().product());
        }
    }

    Err(AocError::BadEntries.into())
}

fn day_1() -> Result<()> {
    let entries = std::fs::read_to_string("res/day_1_input")?
        .lines()
        .map(str::parse::<i32>)
        .collect::<Result<Vec<i32>, _>>()?;

    println!(
        "Day 1, part 1: {}",
        find_product_of_entries_with_sum(&entries, 2, 2020)?
    );
    println!(
        "Day 1, part 2: {}",
        find_product_of_entries_with_sum(&entries, 3, 2020)?
    );

    Ok(())
}

#[derive(Debug)]
struct PasswordEntry {
    occurences: RangeInclusive<usize>,
    required_letter: String,
    password: String,
}

fn parse_password(input: &str) -> IResult<&str, PasswordEntry> {
    let (input, min) = map_res(digit1, str::parse::<usize>)(input)?;
    let (input, _) = char('-')(input)?;
    let (input, max) = map_res(digit1, str::parse::<usize>)(input)?;
    let (input, _) = space1(input)?;

    let (input, required_letter) = alpha1(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = space1(input)?;

    let (input, password) = alpha1(input)?;

    Ok((input, PasswordEntry {
        occurences: min..=max,
        required_letter: required_letter.to_string(),
        password: password.to_string(),
    }))
}

fn day_2() -> Result<()> {
    let input = std::fs::read_to_string("res/day_2_input")?;

    let mut num_valid = 0;

    for line in input.lines() {
        let entry = match parse_password(line) {
            Ok((_input, entry)) => Ok(entry),
            Err(_) => Err(AocError::PasswordParsingError)
        }?;

        if entry.occurences.contains(&entry.password.matches(&entry.required_letter).count()) {
            num_valid = num_valid + 1;
        }
    }

    println!("Day 2, part 1: {}", num_valid);

    Ok(())
}

fn main() -> Result<()> {
    // day_1()?;
    day_2()?;

    Ok(())
}
