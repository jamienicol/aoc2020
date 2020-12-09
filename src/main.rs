use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, anychar, char, digit1, hex_digit1, multispace0, space1},
    combinator::{map, map_res, opt, verify},
    multi::count,
    sequence::{delimited, pair, preceded, separated_pair},
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

fn parse_passport_1(input: &str) -> IResult<&str, ()> {
    let (input, _fields) = permutation((
        delimited(
            multispace0,
            separated_pair(tag("byr"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            multispace0,
        ),
        delimited(
            multispace0,
            separated_pair(tag("iyr"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            multispace0,
        ),
        delimited(
            multispace0,
            separated_pair(tag("eyr"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            multispace0,
        ),
        delimited(
            multispace0,
            separated_pair(tag("hgt"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            multispace0,
        ),
        delimited(
            multispace0,
            separated_pair(tag("hcl"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            multispace0,
        ),
        delimited(
            multispace0,
            separated_pair(tag("ecl"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            multispace0,
        ),
        delimited(
            multispace0,
            separated_pair(tag("pid"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            multispace0,
        ),
        opt(delimited(
            multispace0,
            separated_pair(tag("cid"), tag(":"), take_till(|c| c == ' ' || c == '\n')),
            multispace0,
        )),
    ))(input)?;

    Ok((input, ()))
}

fn parse_passport_2(input: &str) -> IResult<&str, ()> {
    let (input, _fields) = permutation((
        delimited(
            multispace0,
            preceded(
                tag("byr:"),
                verify(map_res(digit1, str::parse::<i32>), |byr| {
                    *byr >= 1920 && *byr <= 2002
                }),
            ),
            multispace0,
        ),
        delimited(
            multispace0,
            preceded(
                tag("iyr:"),
                verify(map_res(digit1, str::parse::<i32>), |iyr| {
                    *iyr >= 2010 && *iyr <= 2020
                }),
            ),
            multispace0,
        ),
        delimited(
            multispace0,
            preceded(
                tag("eyr:"),
                verify(map_res(digit1, str::parse::<i32>), |eyr| {
                    *eyr >= 2020 && *eyr <= 2030
                }),
            ),
            multispace0,
        ),
        delimited(
            multispace0,
            preceded(
                tag("hgt:"),
                verify(
                    pair(
                        map_res(digit1, str::parse::<i32>),
                        alt((tag("cm"), tag("in"))),
                    ),
                    |(hgt, unit)| match *unit {
                        "cm" => *hgt >= 150 && *hgt <= 193,
                        "in" => *hgt >= 59 && *hgt <= 76,
                        _ => unreachable!(),
                    },
                ),
            ),
            multispace0,
        ),
        delimited(
            multispace0,
            preceded(tag("hcl:"), preceded(tag("#"), hex_digit1)),
            multispace0,
        ),
        delimited(
            multispace0,
            preceded(
                tag("ecl:"),
                alt((
                    tag("amb"),
                    tag("blu"),
                    tag("brn"),
                    tag("gry"),
                    tag("grn"),
                    tag("hzl"),
                    tag("oth"),
                )),
            ),
            multispace0,
        ),
        delimited(
            multispace0,
            preceded(
                tag("pid:"),
                verify(digit1, |pid: &str| pid.chars().count() == 9),
            ),
            multispace0,
        ),
        opt(delimited(
            multispace0,
            preceded(tag("cid:"), digit1),
            multispace0,
        )),
    ))(input)?;

    Ok((input, ()))
}

fn day_4() -> Result<()> {
    let input = std::fs::read_to_string("res/day_4_input")?;

    let num_valid_1 = input
        .split("\n\n")
        .map(|input| {
            parse_passport_1(input).map_err(|err| anyhow!("Error parsing passport: {:?}", err))
        })
        .filter_map(Result::ok)
        .count();

    let num_valid_2 = input
        .split("\n\n")
        .map(|input| {
            parse_passport_2(input).map_err(|err| anyhow!("Error parsing passport: {:?}", err))
        })
        .filter_map(Result::ok)
        .count();

    // 196
    println!("Day 4, part 1: {}", num_valid_1);
    // 114
    println!("Day 4, part 2: {}", num_valid_2);

    Ok(())
}

fn parse_seat(input: &str) -> IResult<&str, (i32, i32)> {
    pair(
        map_res(
            count(alt((map(char('F'), |_| '0'), map(char('B'), |_| '1'))), 7),
            |chars| i32::from_str_radix(&chars.into_iter().collect::<String>(), 2),
        ),
        map_res(
            count(alt((map(char('L'), |_| '0'), map(char('R'), |_| '1'))), 3),
            |chars| i32::from_str_radix(&chars.into_iter().collect::<String>(), 2),
        ),
    )(input)
}

fn seat_id(row: i32, column: i32) -> i32 {
    row * 8 + column
}

fn day_5() -> Result<()> {
    let input = std::fs::read_to_string("res/day_5_input")?;

    let passes = input
        .lines()
        .map(|line| {
            Ok(parse_seat(line)
                .map_err(|err| anyhow!("Error parsing seats: {:?}", err))?
                .1)
        })
        .collect::<Result<Vec<_>>>()?;

    let pass_ids = passes
        .iter()
        .map(|(row, column)| seat_id(*row, *column))
        .collect::<Vec<_>>();

    let max_id = pass_ids.iter().max().unwrap();

    // 913
    println!("Day 5, part 1: {}", max_id);

    let all_seats = (0..2i32.pow(7))
        .flat_map(move |row| (0..2i32.pow(3)).map(move |column| (row, column)))
        .collect::<HashSet<(i32, i32)>>();

    let mut empty_seats = all_seats.clone();
    for pass in passes {
        empty_seats.remove(&pass);
    }

    let my_seat = empty_seats
        .iter()
        .find(|(row, column)| {
            let id = seat_id(*row, *column);
            pass_ids.contains(&(id - 1)) && pass_ids.contains(&(id + 1))
        })
        .ok_or_else(|| anyhow!("Couldn't find my seat"))?;
    let my_seat_id = seat_id(my_seat.0, my_seat.1);

    // 717
    println!("Day 5, part 2: {}", my_seat_id);

    Ok(())
}

fn main() -> Result<()> {
    day_1()?;
    day_2()?;
    day_3()?;
    day_4()?;
    day_5()?;

    Ok(())
}
