use std::{collections::HashMap, str::FromStr};

use anyhow::Result;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1, u32},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

pub fn parse_games(input: &str) -> Result<Vec<Game>, ()> {
    let mut parser = separated_list1(line_ending, game);
    if let Ok((_, games)) = parser(input) {
        Ok(games)
    } else {
        Err(())
    }
}

fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = game_id(input)?;
    let (input, subsets) = game_subsets(input)?;
    Ok((input, Game { id, subsets }))
}

fn game_id(input: &str) -> IResult<&str, u32> {
    delimited(tag("Game "), u32, tag(": "))(input)
}

fn game_subsets(input: &str) -> IResult<&str, Vec<Subset>> {
    separated_list1(tag("; "), subset)(input)
}

fn subset(input: &str) -> IResult<&str, Subset> {
    let (input, amounts) = separated_list1(tag(", "), cube_amount)(input)?;
    
    let mut subset = Subset::new();
    for (color, amount) in amounts.iter() {
        subset.insert(*color, *amount);
    }

    Ok((input, subset))
}

fn cube_amount(input: &str) -> IResult<&str, (CubeColor, u32)> {
    let (input, (amount, _, color)) = tuple((u32, space1, color_name))(input)?;
    Ok((input, (color, amount)))
}

fn color_name(input: &str) -> IResult<&str, CubeColor> {
    map_res(alpha1, CubeColor::from_str)(input)
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub subsets: Vec<Subset>,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum CubeColor {
    Red,
    Green,
    Blue,
}

pub type Subset = HashMap<CubeColor, u32>;

impl FromStr for CubeColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(format!("Could not read {} as a color", s)),
        }
    }
}
