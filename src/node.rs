use std::fmt::Display;
use napi_derive::napi;
use super::solver::{
    solver_2015_01, solver_2015_02, solver_2015_03, solver_2015_04, solver_2015_05, solver_2015_06,
    solver_2015_07, solver_2022_01, solver_2022_02, solver_2022_03, solver_2022_04, solver_2022_05,
    solver_2022_06, solver_2022_07, solver_2022_08, solver_2022_09, solver_2022_10, solver_2022_11,
    solver_2022_12, solver_2022_13, solver_2022_14, solver_2022_15, solver_2022_16, solver_2022_17,
    solver_2022_18, solver_2022_20, solver_2022_21, solver_2022_22, solver_2024_01, solver_2024_02,
    solver_2024_03, solver_2024_04, solver_2024_05, solver_2024_06, solver_2024_07,
};

#[napi(object)]
pub struct Answers {
    pub first: String,
    pub second: String,
}

impl Answers {
    pub fn from<T1: Display, T2: Display>((first, second): (T1, T2)) -> Self {
        Self {
            first: format!("{}", first),
            second: format!("{}", second),
        }
    }
}
#[napi]
pub fn solve(year: u32, day: u32) -> Answers {
    match (year, day) {
        (2015, 1) => Answers::from(solver_2015_01::solve()),
        (2015, 2) => Answers::from(solver_2015_02::solve()),
        (2015, 3) => Answers::from(solver_2015_03::solve()),
        (2015, 4) => Answers::from(solver_2015_04::solve()),
        (2015, 5) => Answers::from(solver_2015_05::solve()),
        (2015, 6) => Answers::from(solver_2015_06::solve()),
        (2015, 7) => Answers::from(solver_2015_07::solve()),
        (2022, 1) => Answers::from(solver_2022_01::solve()),
        (2022, 2) => Answers::from(solver_2022_02::solve()),
        (2022, 3) => Answers::from(solver_2022_03::solve()),
        (2022, 4) => Answers::from(solver_2022_04::solve()),
        (2022, 5) => Answers::from(solver_2022_05::solve()),
        (2022, 6) => Answers::from(solver_2022_06::solve()),
        (2022, 7) => Answers::from(solver_2022_07::solve()),
        (2022, 8) => Answers::from(solver_2022_08::solve()),
        (2022, 9) => Answers::from(solver_2022_09::solve()),
        (2022, 10) => Answers::from(solver_2022_10::solve()),
        (2022, 11) => Answers::from(solver_2022_11::solve()),
        (2022, 12) => Answers::from(solver_2022_12::solve()),
        (2022, 13) => Answers::from(solver_2022_13::solve()),
        (2022, 14) => Answers::from(solver_2022_14::solve()),
        (2022, 15) => Answers::from(solver_2022_15::solve()),
        (2022, 16) => Answers::from(solver_2022_16::solve()),
        (2022, 17) => Answers::from(solver_2022_17::solve()),
        (2022, 18) => Answers::from(solver_2022_18::solve()),
        (2022, 20) => Answers::from(solver_2022_20::solve()),
        (2022, 21) => Answers::from(solver_2022_21::solve()),
        (2022, 22) => Answers::from(solver_2022_22::solve()),
        (2024, 1) => Answers::from(solver_2024_01::solve()),
        (2024, 2) => Answers::from(solver_2024_02::solve()),
        (2024, 3) => Answers::from(solver_2024_03::solve()),
        (2024, 4) => Answers::from(solver_2024_04::solve()),
        (2024, 5) => Answers::from(solver_2024_05::solve()),
        (2024, 6) => Answers::from(solver_2024_06::solve()),
        (2024, 7) => Answers::from(solver_2024_07::solve()),
        _ => Answers {
            first: "hello".to_string(),
            second: "bar".to_string(),
        },
    }
}