use crate::utils::{select_group, select_lecture_room, select_teacher, split_days, Day};
use anyhow::{anyhow, Result};
use clap::{builder::PossibleValue, ValueEnum};
use nure_tools::{
    groups::find_group,
    lecture_rooms::find_lecture_room,
    schedule::{get_schedule, Request},
    teachers::find_teacher,
    utils::Period,
};

#[derive(Clone, Debug)]
pub enum SearchType {
    Group,
    Teacher,
    LectureRoom,
}

impl ValueEnum for SearchType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Group, Self::Teacher, Self::LectureRoom]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Group => PossibleValue::new("group"),
            Self::Teacher => PossibleValue::new("teacher"),
            Self::LectureRoom => PossibleValue::new("lecture_room"),
        })
    }
}

pub fn match_schedule(
    search_type: SearchType,
    name: &str,
    week: bool,
    next: bool,
    start_time: Option<String>,
    end_time: Option<String>,
) -> Result<Vec<Day>> {
    let period = match (start_time, end_time, week, next) {
        (None, None, false, false) => Period::now(),
        (None, None, false, true) => Period::next_day(),
        (None, None, true, false) => Period::this_week(),
        (None, None, true, true) => Period::next_week(),
        (Some(start), None, false, false) => match Period::day_from(&start) {
            Ok(value) => value,
            Err(error) => {
                println!("{error}. Will use 'this_day' instead");
                Period::this_day()
            }
        },
        (Some(start), None, true, false) => match Period::week_from(&start) {
            Ok(value) => value,
            Err(error) => {
                println!("{error}. Will use 'this_week' instead");
                Period::this_week()
            }
        },
        (Some(start), Some(end), _, _) => match Period::from_string(&start, &end) {
            Ok(value) => value,
            Err(error) => {
                println!("{error}. Will use 'this_week' instead");
                Period::this_week()
            }
        },
        (_, _, _, _) => {
            println!("Can't parse Period from given parameters, will use 'this_day' instead");
            Period::this_day()
        }
    };
    let schedule = match search_type {
        SearchType::Group => {
            let mut groups = match find_group(name) {
                Ok(value) => value,
                Err(error) => return Err(anyhow!(error)),
            };

            let group = match groups.len() {
                1 => groups.pop().unwrap(),
                _ => select_group(groups),
            };

            get_schedule(Request::Group(group), period).unwrap()
        }
        SearchType::Teacher => {
            let mut teachers = match find_teacher(name) {
                Ok(value) => value,
                Err(error) => return Err(anyhow!(error)),
            };

            let teacher = match teachers.len() {
                1 => teachers.pop().unwrap(),
                _ => select_teacher(teachers),
            };

            get_schedule(Request::Teacher(teacher), period).unwrap()
        }
        SearchType::LectureRoom => {
            let mut lecture_rooms = match find_lecture_room(name) {
                Ok(value) => value,
                Err(error) => return Err(anyhow!(error)),
            };

            let lecture_room = match lecture_rooms.len() {
                1 => lecture_rooms.pop().unwrap(),
                _ => select_lecture_room(lecture_rooms),
            };

            get_schedule(Request::LectureRoom(lecture_room), period).unwrap()
        }
    };

    split_days(schedule)
}
