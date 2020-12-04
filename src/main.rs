use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    character::complete::{alpha1, anychar, char, digit1, space1},
    combinator::map_res,
    IResult,
};

use std::ops::RangeInclusive;

fn find_product_of_entries_with_sum(entries: &[i32], num_entries: usize, sum: i32) -> Result<i32> {
    for combination in entries.iter().combinations(num_entries) {
        if combination.iter().cloned().sum::<i32>() == sum {
            return Ok(combination.iter().cloned().product());
        }
    }

    Err(anyhow!(
        "No {} entries exist which sum to {}",
        num_entries,
        sum
    ))
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
    required_letter: char,
    password: String,
}

impl PasswordEntry {
    fn valid_old(&self) -> Result<bool> {
        Ok(self
            .occurences
            .contains(&self.password.matches(self.required_letter).count()))
    }

    fn valid_new(&self) -> Result<bool> {
        let start = self
            .password
            .chars()
            .nth(self.occurences.start() - 1)
            .ok_or_else(|| {
                anyhow!(
                    "Password {:?} has no char at index {}",
                    self.password,
                    self.occurences.start() - 1
                )
            })?;

        let end = self
            .password
            .chars()
            .nth(self.occurences.end() - 1)
            .ok_or_else(|| {
                anyhow!(
                    "Password {:?} has no char at index {}",
                    self.password,
                    self.occurences.end() - 1
                )
            })?;

        Ok((start == self.required_letter) ^ (end == self.required_letter))
    }
}

fn parse_password(input: &str) -> IResult<&str, PasswordEntry> {
    let (input, min) = map_res(digit1, str::parse::<usize>)(input)?;
    let (input, _) = char('-')(input)?;
    let (input, max) = map_res(digit1, str::parse::<usize>)(input)?;
    let (input, _) = space1(input)?;

    let (input, required_letter) = anychar(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = space1(input)?;

    let (input, password) = alpha1(input)?;

    Ok((
        input,
        PasswordEntry {
            occurences: min..=max,
            required_letter,
            password: password.to_string(),
        },
    ))
}

fn day_2() -> Result<()> {
    let input = std::fs::read_to_string("res/day_2_input")?;

    let num_valid = input
        .lines()
        .map(parse_password)
        .try_fold((0, 0), |valid_counts, entry| match entry {
            Ok((_input, entry)) => Ok((
                valid_counts.0 + entry.valid_old()? as i32,
                valid_counts.1 + entry.valid_new()? as i32,
            )),
            Err(err) => Err(anyhow!("Error parsing password file: {:?}", err)),
        })?;

    println!("Day 2, part 1: {}", num_valid.0);
    println!("Day 2, part 2: {}", num_valid.1);

    Ok(())
}

fn main() -> Result<()> {
    // day_1()?;
    day_2()?;

    Ok(())
}
