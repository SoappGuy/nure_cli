use clap::Subcommand;
use color_eyre::Result;
use nure_tools::{groups::find_group, lecture_rooms::find_lecture_room, teachers::find_teacher};

#[derive(Subcommand)]
pub enum Search {
    /// Search for group
    #[command(arg_required_else_help = true)]
    Group { name: String },
    /// Search for teacher
    #[command(arg_required_else_help = true)]
    Teacher { name: String },
    /// Search for lecture-room
    #[command(arg_required_else_help = true)]
    LectureRoom { name: String },
}

pub fn match_search(search_type: Option<Search>) -> Result<()> {
    match search_type {
        Some(Search::Group { name }) => {
            let mut groups = find_group(name.as_str())?;
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
        Some(Search::Teacher { name }) => {
            let mut teachers = find_teacher(name.as_str())?;
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
        Some(Search::LectureRoom { name }) => {
            let mut lecture_rooms = find_lecture_room(name.as_str())?;
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
        None => {
            println!("What to search?, see `nure_cli search help`");
        }
    }

    Ok(())
}
