use anyhow::{anyhow, Result};
use chrono::DateTime;
use chrono_tz::Tz;
use core::fmt;
use dialoguer::Select;
use nure_tools::{groups::Group, lecture_rooms::LectureRoom, schedule::Lecture, teachers::Teacher};
use runtime_format::{FormatArgs, FormatKey, FormatKeyError};

// pub fn split_days(schedule: Vec<Lecture>) -> Result<Vec<Day>> {
//     let mut chunk;
//     if !schedule.is_empty() {
//         chunk = Day::with_date(schedule[0].period.start_time);
//     } else {
//         return Err(anyhow!("given schedule is empty"));
//     }
//
//     let mut splitted = vec![];
//
//     for lecture in schedule {
//         if lecture.period.start_time.date_naive() == chunk.date.date_naive() {
//             chunk.lectures.push(lecture);
//         } else {
//             if !chunk.lectures.is_empty() {
//                 splitted.push(chunk)
//             }
//
//             chunk = Day::with_date(lecture.period.start_time);
//         }
//     }
//
//     if !chunk.lectures.is_empty() {
//         splitted.push(chunk);
//     }
//
//     Ok(splitted)
// }

pub fn split_days(schedule: Vec<Lecture>) -> Result<Vec<Day>> {
    let mut daily_chunks: Vec<Day> = vec![];

    let start_date;
    let end_date;

    if !schedule.is_empty() {
        start_date = schedule[0].period.start_time;
        end_date = schedule[schedule.len() - 1].period.end_time;
    } else {
        return Err(anyhow!("No lectures (hell yeah, we balling)"));
    }
    let mut date = start_date;

    while date <= end_date {
        daily_chunks.push(Day::with_date(date));
        date += chrono::Duration::days(1);
    }

    for lecture in schedule {
        let idx =
            (lecture.period.start_time.date_naive() - start_date.date_naive()).num_days() as usize;

        if idx < daily_chunks.len() {
            daily_chunks[idx].lectures.push(lecture);
        }
    }

    Ok(daily_chunks)
}

pub fn format_string(format: &str, lecture_separator: &str, day_lable: &str, schedule: Vec<Day>) {
    let mut formated;
    let foramted = format!("{{{}}}", day_lable);
    let day_lable = foramted.as_str();
    for day in schedule {
        println!("{}", FormatArgs::new(day_lable, &day));
        formated = String::new();
        if day.lectures.is_empty() {
            println!("No lectures{lecture_separator}")
        } else {
            for lecture in day.lectures {
                let wrapped = LectureWrapper { lecture };
                let result = FormatArgs::new(format, &wrapped);
                formated += &result.to_string();
                formated += lecture_separator;
            }
            println!("{formated}")
        }
    }
}

pub fn select_group(groups: Vec<Group>) -> Group {
    let selection = Select::new()
        .with_prompt("Select group (↑/↓)")
        .items(&groups)
        .interact()
        .unwrap();

    groups.into_iter().nth(selection).unwrap()
}

pub fn select_teacher(teachers: Vec<Teacher>) -> Teacher {
    let selection = Select::new()
        .with_prompt("Select teacher (↑/↓)")
        .items(&teachers)
        .interact()
        .unwrap();

    teachers.into_iter().nth(selection).unwrap()
}

pub fn select_lecture_room(lecture_rooms: Vec<LectureRoom>) -> LectureRoom {
    let selection = Select::new()
        .with_prompt("Select lecture_room (↑/↓)")
        .items(&lecture_rooms)
        .interact()
        .unwrap();

    lecture_rooms.into_iter().nth(selection).unwrap()
}

pub struct Day {
    pub lectures: Vec<Lecture>,
    pub date: DateTime<Tz>,
}

impl Day {
    pub fn with_date(date_: DateTime<Tz>) -> Self {
        let lectures = vec![];
        let date = date_;
        Self { lectures, date }
    }
}

impl FormatKey for Day {
    fn fmt(
        &self,
        key: &str,
        f: &mut fmt::Formatter<'_>,
    ) -> std::prelude::v1::Result<(), FormatKeyError> {
        write!(f, "{}", self.date.format(key)).map_err(FormatKeyError::Fmt)
    }
}

struct LectureWrapper {
    lecture: Lecture,
}

impl FormatKey for LectureWrapper {
    fn fmt(&self, key: &str, f: &mut fmt::Formatter<'_>) -> Result<(), FormatKeyError> {
        match key {
            "lecture_room" => {
                write!(f, "{}", self.lecture.lecture_room).map_err(FormatKeyError::Fmt)
            }
            "start_time" => write!(f, "{}", self.lecture.period.start_time.format("%H:%M"))
                .map_err(FormatKeyError::Fmt),
            "end_time" => write!(f, "{}", self.lecture.period.end_time.format("%H:%M"))
                .map_err(FormatKeyError::Fmt),
            "number_pair" => write!(f, "{}", self.lecture.number_pair).map_err(FormatKeyError::Fmt),
            "lecture_type" => {
                write!(f, "{}", self.lecture.lecture_type).map_err(FormatKeyError::Fmt)
            }
            "subject.brief" => {
                write!(f, "{}", self.lecture.subject.brief).map_err(FormatKeyError::Fmt)
            }
            "subject.id" => write!(f, "{}", self.lecture.subject.id).map_err(FormatKeyError::Fmt),
            "subject.title" => {
                write!(f, "{}", self.lecture.subject.title).map_err(FormatKeyError::Fmt)
            }
            _ => write!(f, "{}", { "#wtf?" }).map_err(FormatKeyError::Fmt),
        }
    }
}
