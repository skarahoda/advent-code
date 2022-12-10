extern crate core;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use clap::{Parser, ValueEnum};
mod solver;
use solver::{
    solver_2015_01,
    solver_2015_02,
    solver_2015_03,
    solver_2015_04,
    solver_2015_05,
    solver_2015_06,
    solver_2015_07,
    solver_2022_01,
    solver_2022_02,
    solver_2022_03,
    solver_2022_04,
    solver_2022_05,
    solver_2022_06,
    solver_2022_07,
    solver_2022_08,
    solver_2022_09,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Year {
    #[value(name="2015")]
    Year2015,
    #[value(name="2022")]
    Year2022,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Day {
    #[value(name="1")]
    Day1,
    #[value(name="2")]
    Day2,
    #[value(name="3")]
    Day3,
    #[value(name="4")]
    Day4,
    #[value(name="5")]
    Day5,
    #[value(name="6")]
    Day6,
    #[value(name="7")]
    Day7,
    #[value(name="8")]
    Day8,
    #[value(name="9")]
    Day9,
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_enum)]
    year: Year,

    #[arg(value_enum)]
    day: Day,
}

fn main() {
    let args = Cli::parse();
    match (args.year, args.day) {
        (Year::Year2015, Day::Day1) => println!("{:?}", solver_2015_01::solve()),
        (Year::Year2015, Day::Day2) => println!("{:?}", solver_2015_02::solve()),
        (Year::Year2015, Day::Day3) => println!("{:?}", solver_2015_03::solve()),
        (Year::Year2015, Day::Day4) => println!("{:?}", solver_2015_04::solve()),
        (Year::Year2015, Day::Day5) => println!("{:?}", solver_2015_05::solve()),
        (Year::Year2015, Day::Day6) => println!("{:?}", solver_2015_06::solve()),
        (Year::Year2015, Day::Day7) => println!("{:?}", solver_2015_07::solve()),
        (Year::Year2022, Day::Day1) => println!("{:?}", solver_2022_01::solve()),
        (Year::Year2022, Day::Day2) => println!("{:?}", solver_2022_02::solve()),
        (Year::Year2022, Day::Day3) => println!("{:?}", solver_2022_03::solve()),
        (Year::Year2022, Day::Day4) => println!("{:?}", solver_2022_04::solve()),
        (Year::Year2022, Day::Day5) => println!("{:?}", solver_2022_05::solve()),
        (Year::Year2022, Day::Day6) => println!("{:?}", solver_2022_06::solve()),
        (Year::Year2022, Day::Day7) => println!("{:?}", solver_2022_07::solve()),
        (Year::Year2022, Day::Day8) => println!("{:?}", solver_2022_08::solve()),
        (Year::Year2022, Day::Day9) => println!("{:?}", solver_2022_09::solve()),
        _ => panic!("Puzzle is not solved yet!")
    }
}
