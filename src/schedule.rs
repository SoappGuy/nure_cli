use color_eyre::Result;
use nure_tools::{
    groups::find_exect_group,
    lecture_rooms::find_exect_lecture_room,
    schedule::{get_schedule, Lecture, Request},
    teachers::find_exect_teacher,
    utils::Period,
};

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Schedule {
    /// Schedule for group. If no parameters were added - returns the entire schedule for the current day.
    #[command(arg_required_else_help = true)]
    Group {
        /// Name to search.
        name: String,
        /// If set will get schedule for week around <START> time or around current/next day.
        #[arg(short, long = "week", action = clap::ArgAction::SetTrue)]
        week: bool,
        /// If set will search for next day/week.
        #[arg(short, long = "next", action = clap::ArgAction::SetTrue)]
        next: bool,
        /// Sets period start-time.
        #[arg(short, long = "start")]
        start_time: Option<String>,
        /// sets period end-time.
        #[arg(short, long = "end")]
        end_time: Option<String>,
    },
    /// Schedule for teacher. If no parameters were added - returns the entire schedule for the current day.
    #[command(arg_required_else_help = true)]
    Teacher {
        /// Name to search.
        name: String,
        /// If set will get schedule for week around <START> time or around current/next day.
        #[arg(short, long = "week", action = clap::ArgAction::SetTrue)]
        week: bool,
        /// If set will search for next day/week.
        #[arg(short, long = "next", action = clap::ArgAction::SetTrue)]
        next: bool,
        /// Sets period start-time.
        #[arg(short, long = "start")]
        start_time: Option<String>,
        /// sets period end-time.
        #[arg(short, long = "end")]
        end_time: Option<String>,
    },
    /// Schedule for lecture-room. If no parameters were added - returns the entire schedule for the current day.
    #[command(arg_required_else_help = true)]
    LectureRoom {
        /// Name to search.
        name: String,
        /// If set will get schedule for week around <START> time or around current/next day.
        #[arg(short, long = "week", action = clap::ArgAction::SetTrue)]
        week: bool,
        /// If set will search for next day/week.
        #[arg(short, long = "next", action = clap::ArgAction::SetTrue)]
        next: bool,
        /// Sets period start-time.
        #[arg(short, long = "start")]
        start_time: Option<String>,
        /// sets period end-time.
        #[arg(short, long = "end")]
        end_time: Option<String>,
    },
}

// pub fn match_schedule(
//     search_type: Option<Schedule>,
//     start_time: Option<String>,
//     end_time: Option<String>,
//     format: Option<String>,
//     separator: Option<String>,
//     curr_week: Option<bool>,
//     next_week: Option<bool>,
// ) -> Result<()> {
//     match search_type {
//         Some(Schedule::Group { name }) => {
//             let mut groups = find_group(name.as_str())?;
//             match groups.len() {
//                 0 => {
//                     println!("There is no groups with name '{}'", name);
//                 }
//                 1 => {
//                     let group = groups.pop().unwrap();
//                     let schedule = get_schedule(Request::Group(group), start_time, end_time)?;
//
//                     if !schedule.is_empty() {
//                         println!("{}", format_string(format, separator, schedule));
//                     } else {
//                         println!("There is no Lectures for given period");
//                     }
//                 }
//                 _ => {
//                     println!("There is too many valid groups for name {}, do search to get exect name of your group", name);
//                 }
//             }
//         }
//         Some(Schedule::Teacher { name }) => {
//             let mut teachers = find_teacher(name.as_str())?;
//             match teachers.len() {
//                 0 => {
//                     println!("There is no teachers with name '{}'", name);
//                 }
//                 1 => {
//                     let teacher = teachers.pop().unwrap();
//                     let schedule = get_schedule(Request::Teacher(teacher), start_time, end_time)?;
//
//                     if !schedule.is_empty() {
//                         println!("{}", format_string(format, separator, schedule));
//                     } else {
//                         println!("There is no Lectures for given period");
//                     }
//                 }
//                 _ => {
//                     println!("There is too many valid teachers for name {}, do search to get exect name of teacher", name);
//                 }
//             }
//         }
//         Some(Schedule::LectureRoom { name }) => {
//             let mut lecture_rooms = find_lecture_room(name.as_str())?;
//             match lecture_rooms.len() {
//                 0 => {
//                     println!("There is no lecture_rooms with name '{}'", name);
//                 }
//                 1 => {
//                     let lecture_room = lecture_rooms.pop().unwrap();
//                     let schedule =
//                         get_schedule(Request::LectureRoom(lecture_room), start_time, end_time)?;
//
//                     if !schedule.is_empty() {
//                         println!("{}", format_string(format, separator, schedule));
//                     } else {
//                         println!("There is no Lectures for given period");
//                     }
//                 }
//                 _ => {
//                     println!("There is too many valid lecture_rooms for name {}, do search to get exect name of your lecture_room", name);
//                 }
//             }
//         }
//         None => {
//             println!("Please provide a type to get schedule for, see `nure_cli schedule help`");
//         }
//     }
//     Ok(())
// }

pub fn match_schedule(search_type: Option<Schedule>) -> Result<Vec<Lecture>> {
    match search_type {
        Some(Schedule::Group {
            name,
            week,
            next,
            start_time,
            end_time,
        }) => {
            let period = match (start_time, end_time, week, next) {
                (None, None, false, false) => Period::this_day(),
                (None, None, false, true) => Period::next_day(),
                (None, None, true, false) => Period::this_week(),
                (None, None, true, true) => Period::next_week(),
                (Some(start), None, false, false) => Period::day_from(&start),
                (Some(start), None, true, false) => Period::week_from(&start),
                (Some(start), Some(end), _, _) => Period::from_string(&start, &end),
                (_, _, _, _) => {
                    println!("Can't parse Period from given parameters, will use 'this_day'.");
                    Period::this_day()
                }
            };

            let group = find_exect_group(&name).unwrap();

            return Ok(get_schedule(Request::Group(group), period).unwrap());
        }
        Some(Schedule::Teacher {
            name,
            week,
            next,
            start_time,
            end_time,
        }) => {
            let period = match (start_time, end_time, week, next) {
                (None, None, false, false) => Period::this_day(),
                (None, None, false, true) => Period::next_day(),
                (None, None, true, false) => Period::this_week(),
                (None, None, true, true) => Period::next_week(),
                (Some(start), None, false, false) => Period::day_from(&start),
                (Some(start), None, true, false) => Period::week_from(&start),
                (Some(start), Some(end), _, _) => Period::from_string(&start, &end),
                (_, _, _, _) => {
                    println!("Can't parse Period from given parameters, will use 'this_day'.");
                    Period::this_day()
                }
            };

            let teacher = find_exect_teacher(&name).unwrap();

            return Ok(get_schedule(Request::Teacher(teacher), period).unwrap());
        }
        Some(Schedule::LectureRoom {
            name,
            week,
            next,
            start_time,
            end_time,
        }) => {
            let period = match (start_time, end_time, week, next) {
                (None, None, false, false) => Period::this_day(),
                (None, None, false, true) => Period::next_day(),
                (None, None, true, false) => Period::this_week(),
                (None, None, true, true) => Period::next_week(),
                (Some(start), None, false, false) => Period::day_from(&start),
                (Some(start), None, true, false) => Period::week_from(&start),
                (Some(start), Some(end), _, _) => Period::from_string(&start, &end),
                (_, _, _, _) => {
                    println!("Can't parse Period from given parameters, will use 'this_day'.");
                    Period::this_day()
                }
            };

            let lecture_room = find_exect_lecture_room(&name).unwrap();

            return Ok(get_schedule(Request::LectureRoom(lecture_room), period).unwrap());
        }
        None => {
            println!("Please provide a type to search. See `nure_cli schedule help`");
        }
    }
    Ok(vec![])
}
