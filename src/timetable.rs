use serde::{Deserialize, Serialize};
use std::fs;

use chrono::Timelike;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lesson {
    pub subject: String,
    pub room: String,
    pub teacher: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Day {
    pub _1: Lesson,
    pub _2: Lesson,
    pub _3: Lesson,
    pub _4: Lesson,
    pub _5: Lesson,
    pub _6: Lesson,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Week {
    pub mon: Day,
    pub tue: Day,
    pub wed: Day,
    pub thu: Day,
    pub fri: Day,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timetable {
    pub a: Week,
    pub b: Week,
}

fn name_line_gen(name: String) -> String {
    format!("│{: ^22}│", name)
}

fn lesson_cell(lesson: Lesson) -> [String; 7] {
    let mut lines: [String; 7] = Default::default();

    lines[0] = String::from("┌──────────────────────┐");

    lines[1] = name_line_gen(lesson.subject);

    lines[2] = String::from("│                      │");

    lines[3] = name_line_gen(lesson.room);

    lines[4] = String::from("│                      │");

    lines[5] = name_line_gen(lesson.teacher);

    lines[6] = String::from("└──────────────────────┘");

    lines
}

pub fn draw_day(day: Day) -> [[String; 7]; 6] {
    let mut lines: [[String; 7]; 6] = Default::default();

    for i in 0..6 {
        let lesson = match i {
            0 => day.clone()._1,
            1 => day.clone()._2,
            2 => day.clone()._3,
            3 => day.clone()._4,
            4 => day.clone()._5,
            5 => day.clone()._6,
            _ => Lesson {
                subject: String::from(""),
                room: String::from(""),
                teacher: String::from(""),
            },
        };

        lines[i] = lesson_cell(lesson);
    }

    lines
}

pub fn load_timetable(path: &str) -> Result<Timetable, String> {
    let file_contents = fs::read_to_string(path).expect("Error Reading Timetable File!");

    match serde_json::from_str(file_contents.as_str()) {
        Ok(timetable) => Ok(timetable),
        Err(e) => Err(format!(
            "\n\x1b[31mError reading timetable\x1b[0m:\n\t- {}",
            e,
        )),
    }
}

pub fn load_last_aweek(path: &str) -> Result<chrono::NaiveDate, String> {
    let mut file_contents = fs::read_to_string(path).expect("Error Reading Timetable File!");

    file_contents.pop();

    let date = chrono::NaiveDate::parse_from_str(&file_contents, "%Y/%m/%d");

    match date {
        Ok(d) => Ok(d),
        Err(e) => Err(format!(
            "\n\x1b[31mError reading timetable\x1b[0m:\n\t- {}",
            e,
        )),
    }
}

pub fn num_of_weeks_since(date_1: chrono::NaiveDate, date_2: chrono::NaiveDate) -> i64 {
    (date_2 - date_1).num_weeks()
}

pub fn get_period_from_time(time: chrono::NaiveTime) -> i8 {
    let hour = time.hour();
    match hour {
        9 => 0,
        10 => 1,
        11 => 2,
        12 => 3,
        13 => 4,
        14 => 5,
        _ => -1,
    }
}
