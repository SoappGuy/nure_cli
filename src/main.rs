mod schedule;
mod search;
mod utils;

use std::u8;

use crate::{
    schedule::{match_schedule, SearchType},
    search::{match_search, Search},
    utils::format_string,
};
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author="dumbnerd", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Try to find object of your choice
    Search {
        /// Choice object type
        #[arg(value_enum)]
        search_type: Search,
        /// Name to search.
        name: String,
    },
    /// Get schedule with given parameters
    Schedule {
        /// Type to search.
        #[arg(value_enum)]
        search_type: SearchType,
        /// Name to search.
        name: String,
        /** Limit for pairs to print.
        Default = 255
        */
        #[arg(short, long = "limit", verbatim_doc_comment)]
        limit: Option<u8>,
        /// If set will get schedule for week around <START> time
        /// or around current/next day.
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

        /** Output format for each Lecture.
                ​
        Default = "{number_pair}: {subject.brief} - {lecture_type}"
        ​
        Available fields:
        ​
        number_pair - sequence number of Lecture.
        start_time - Lecture start-time.
        end_time - Lecture end-time.
        lecture_type - type of Lecture.
        lecture_room - name of lecture-room.
        subject - subject oriented stuff.
            subject.brief - short name of subject.
            subject.title - long name of subject.
            subject.id - id of subject.
        ​
        */
        #[arg(long = "fmt", verbatim_doc_comment)]
        format: Option<String>,
        /** String for Lecture separation.
        Default = "\n"
        */
        #[arg(long = "sep", verbatim_doc_comment)]
        lecture_separator: Option<String>,
        /** Each Day label.
        Default = "{%A}"
        */
        #[arg(long = "label", verbatim_doc_comment)]
        day_label: Option<String>,
        /// If set will get schedule for week around <START> time
        /// or around current/next day.
        #[arg(long = "no_labels", action = clap::ArgAction::SetTrue)]
        no_labels: bool,
    },
}

fn main() -> Result<()> {
    let commands = Cli::parse();

    match commands.command {
        Some(Commands::Search { search_type, name }) => {
            match match_search(search_type, &name) {
                Ok(_) => (),
                Err(error) => return Ok(println!("{}", error)),
            };
        }
        Some(Commands::Schedule {
            search_type,
            format,
            lecture_separator,
            name,
            week,
            next,
            start_time,
            end_time,
            day_label,
            limit,
            no_labels,
        }) => {
            let format =
                &format.unwrap_or("{number_pair}: {subject.brief} - {lecture_type}".to_string());

            let lecture_separator = &lecture_separator
                .unwrap_or("\n".to_string())
                .replace("\\n", "\n");

            let temp = day_label.unwrap_or("{%A}".to_string());
            let day_label_format = if !no_labels {
                Some(temp.as_str())
            } else {
                None
            };

            let schedule =
                match match_schedule(search_type, &name, week, next, start_time, end_time) {
                    Ok(value) => value,
                    Err(error) => return Ok(println!("{}", error)),
                };

            let limit = limit.unwrap_or(u8::MAX);

            format_string(format, lecture_separator, day_label_format, limit, schedule);
        }
        None => {
            println!("Please provide a valid command, see `nure_cli help`");
        }
    }

    Ok(())
}
