use anyhow::Result;
use itertools::Itertools;
use thiserror::Error;

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Error, Debug)]
enum AocError {
    #[error("No entries exist with a matching sum")]
    BadEntries,
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
    let file = File::open("res/day_1_input")?;
    let reader = BufReader::new(file);
    let entries = reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>())
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

fn main() -> Result<()> {
    day_1()?;

    Ok(())
}
