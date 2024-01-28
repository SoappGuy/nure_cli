mod schedule;
mod search;
mod utils;

use crate::{
    schedule::{match_schedule, Schedule},
    search::{match_search, Search},
    utils::format_string,
};

use clap::{Parser, Subcommand};
use color_eyre::Result;

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
        #[command(subcommand)]
        search_type: Option<Search>,
    },
    /// Get schedule with given parameters
    Schedule {
        /// Choice object type
        #[command(subcommand)]
        search_type: Option<Schedule>,

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
        #[arg(short, long = "fmt", verbatim_doc_comment)]
        format: Option<String>,
        /** String for Lecture separation.
        Default = "\n"
        */
        #[arg(short, long = "sep", verbatim_doc_comment)]
        separator: Option<String>,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let commands = Cli::parse();

    match commands.command {
        Some(Commands::Search { search_type }) => {
            match_search(search_type)?;
        }
        Some(Commands::Schedule {
            search_type,
            format,
            separator,
        }) => {
            // match_schedule(search_type, start_time, end_time, format, separator)?;
            let schedule = match_schedule(search_type)?;

            println!("{}", format_string(format, separator, schedule));
        }
        None => {
            println!("Please provide a valid command, see `nure_cli help`");
        }
    }

    Ok(())
}
