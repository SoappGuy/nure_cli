use core::fmt;
use nure_tools::schedule::Lecture;
use runtime_format::{FormatArgs, FormatKey, FormatKeyError};

struct LectureWrapper {
    lecture: Lecture,
}

impl FormatKey for LectureWrapper {
    fn fmt(&self, key: &str, f: &mut fmt::Formatter<'_>) -> Result<(), FormatKeyError> {
        match key {
            "lecture_room" => {
                write!(f, "{}", self.lecture.lecture_room).map_err(FormatKeyError::Fmt)
            }
            "start_time" => {
                write!(f, "{}", self.lecture.period.start_time).map_err(FormatKeyError::Fmt)
            }
            "end_time" => {
                write!(f, "{}", self.lecture.period.end_time).map_err(FormatKeyError::Fmt)
            }
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

pub fn format_string(
    format: Option<String>,
    separator: Option<String>,
    schedule: Vec<Lecture>,
) -> String {
    let mut formated = String::new();
    let format = format.unwrap_or("{number_pair}: {subject.brief} - {lecture_type}".to_string());
    let separator = separator.unwrap_or("\n".to_string()).replace("\\n", "\n");

    for lecture in schedule {
        let wrapped = LectureWrapper { lecture };
        let result = FormatArgs::new(format.as_str(), &wrapped);
        formated += &result.to_string();
        formated += &separator;
    }

    formated
}
