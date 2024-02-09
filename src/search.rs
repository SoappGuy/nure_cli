use anyhow::{anyhow, Result};
use clap::{builder::PossibleValue, ValueEnum};
use nure_tools::{groups::find_group, lecture_rooms::find_lecture_room, teachers::find_teacher};

// #[derive(Subcommand)]
// pub enum Search {
//     /// Search for group
//     #[command(arg_required_else_help = true)]
//     Group { name: String },
//     /// Search for teacher
//     #[command(arg_required_else_help = true)]
//     Teacher { name: String },
//     /// Search for lecture-room
//     #[command(arg_required_else_help = true)]
//     LectureRoom { name: String },
// }

#[derive(Clone, Debug)]
pub enum Search {
    Group,
    Teacher,
    LectureRoom,
}

impl ValueEnum for Search {
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

pub fn match_search(search_type: Search, name: &str) -> Result<()> {
    match search_type {
        Search::Group => {
            let mut groups = match find_group(name) {
                Ok(value) => value,
                Err(error) => return Err(anyhow!(error)),
            };
            match groups.len() {
                0 => {
                    println!("Didn't find any groups matching '{}'", name);
                }
                1 => {
                    let group = groups.pop().unwrap();
                    println!("Found 1 group with name {}", group.name);
                }
                len => {
                    println!("Found {} groups with names:", len);
                    for group in groups {
                        println!("\t{}", group.name);
                    }
                }
            }
        }
        Search::Teacher => {
            let mut teachers = match find_teacher(name) {
                Ok(value) => value,
                Err(error) => return Err(anyhow!(error)),
            };
            match teachers.len() {
                0 => {
                    println!("Didn't find any teachers matching '{}'", name);
                }
                1 => {
                    let teacher = teachers.pop().unwrap();
                    println!("Found 1 teacher with name {}", teacher.full_name);
                }
                len => {
                    println!("Found {} teachers with names:", len);
                    for teacher in teachers {
                        println!("\t{}", teacher.full_name);
                    }
                }
            }
        }
        Search::LectureRoom => {
            let mut lecture_rooms = match find_lecture_room(name) {
                Ok(value) => value,
                Err(error) => return Err(anyhow!(error)),
            };
            match lecture_rooms.len() {
                0 => {
                    println!("Didn't find any lecture_rooms matching '{}'", name);
                }
                1 => {
                    let lecture_room = lecture_rooms.pop().unwrap();
                    println!("Found 1 lecture_room with name {}", lecture_room.name);
                }
                len => {
                    println!("Found {} lecture_rooms with names:", len);
                    for lecture_room in lecture_rooms {
                        println!("\t{}", lecture_room.name);
                    }
                }
            }
        }
    }

    Ok(())
}
