use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, anychar, char, digit1, multispace1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use std::collections::HashSet;
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

fn num_trees_encountered(map: &[bool], map_size: (usize, usize), slope: (usize, usize)) -> usize {
    let mut x = 0;
    let mut tree_count = 0;

    for y in (slope.1..map_size.1).step_by(slope.1) {
        x = (x + slope.0) % map_size.0;
        if map[x + y * map_size.0] {
            tree_count = tree_count + 1;
        }
    }

    tree_count
}

fn day_3() -> Result<()> {
    let input = std::fs::read_to_string("res/day_3_input")?;

    let map_height = input.lines().count();
    let map_width = input.lines().nth(0).unwrap().chars().count();

    let map = input
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '.' => Ok(false),
            '#' => Ok(true),
            char => Err(anyhow!("Unexpected input {:?} in tree map", char)),
        })
        .collect::<Result<Vec<bool>>>()?;
    assert_eq!(
        map_width * map_height,
        map.len(),
        "Unexpected size of tree map"
    );

    let tree_count = num_trees_encountered(&map, (map_width, map_height), (3, 1));
    println!("Day 3, part 1: {}", tree_count);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product: usize = slopes
        .iter()
        .map(|slope| num_trees_encountered(&map, (map_width, map_height), *slope))
        .product();

    println!("Day 3, part 2: {}", product);

    Ok(())
}

fn parse_passport_1(input: &str) -> IResult<&str, HashSet<&str>> {
    let (input, fields) = separated_list1(
        multispace1,
        alt((
            separated_pair(tag("byr"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            separated_pair(tag("iyr"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            separated_pair(tag("eyr"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            separated_pair(tag("hgt"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            separated_pair(tag("hcl"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            separated_pair(tag("ecl"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            separated_pair(tag("pid"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            separated_pair(tag("cid"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
        )),
    )(input)?;

    let mut passport = HashSet::default();
    for (field, _value) in fields {
        passport.insert(field);
    }

    Ok((input, passport))
}

fn day_4() -> Result<()> {
    let input = std::fs::read_to_string("res/day_4_input")?;

    const REQUIRED_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let num_valid = input
        .split("\n\n")
        .map(|input| {
            parse_passport_1(input).map_err(|err| anyhow!("Error parsing passport: {:?}", err))
        })
        .flatten()
        .map(|(_input, passport)| {
            REQUIRED_FIELDS.iter().all(|field| passport.contains(field)) as usize
        })
        .sum::<usize>();

    // 196
    println!("Day 4, part 1: {}", num_valid);

    Ok(())
}

fn main() -> Result<()> {
    day_1()?;
    day_2()?;
    day_3()?;
    day_4()?;

    Ok(())
}
