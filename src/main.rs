use serde::{Deserialize, Serialize};
use serde_json::json;

use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Lesson {
    Subject: String,
    Room: String,
    Teacher: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Day {
    _1: Lesson,
    _2: Lesson,
    _3: Lesson,
    _4: Lesson,
    _5: Lesson,
    _6: Lesson,
}

#[derive(Serialize, Deserialize, Debug)]
struct Week {
    mon: Day,
    tue: Day,
    wed: Day,
    thu: Day,
    fri: Day,
}

#[derive(Serialize, Deserialize, Debug)]
struct Timetable {
    a: Week,
    b: Week,
}

fn load_timetable(path: $str) -> Timetable{
    
}

fn main() {}
