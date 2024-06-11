use termion::raw::IntoRawMode;
use termion::{async_stdin, clear, color, cursor, terminal_size};

use chrono::Datelike;

use std::io::{stdout, Read, Write};
use std::time::Duration;
use std::{process, thread};

mod timetable;

fn exit_app() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}{}",
        cursor::Goto(1, 1),
        clear::All,
        cursor::Show
    )
    .unwrap();
    stdout.flush().unwrap();

    process::exit(0);
}

fn flood_screen(color: (u8, u8, u8)) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let (width, height) = terminal_size().unwrap();

    for y in 1..=height {
        write!(
            stdout,
            "{}{}{}{}",
            cursor::Goto(1, y),
            color::Bg(color::Rgb(color.0, color.1, color.2)),
            (0..width).map(|_| " ").collect::<String>(),
            color::Bg(color::Reset)
        )
        .unwrap();
    }
}

fn draw_menu(choices: Vec<&str>, selected: i8) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (width, height) = terminal_size().unwrap();

    let (menu_width, menu_height) = (25u16, (6 + choices.len()) as u16);

    if selected >= choices.len() as i8 {
        write!(
            stdout,
            "{}{}{}Error displaying menu:\n\t- Selected value outside bounds{}",
            clear::All,
            cursor::Goto(1, 1),
            color::Fg(color::Red),
            color::Fg(color::Reset)
        )
        .unwrap();
        stdout.flush().unwrap();

        process::exit(1);
    }

    let mut x_offset = 0;
    let mut y_offset = 0;

    if !(menu_width > width || menu_height > height) {
        x_offset = (width - menu_width) / 2;
        y_offset = (height - menu_height) / 2;
    }

    let mut lines: Vec<String> = vec![
        format!(
            "{goto}{colourBg}{colourFg}{lines}┤Timetable├{lines}{resetFg}",
            goto = cursor::Goto(x_offset, y_offset),
            colourBg = color::Bg(color::LightBlue),
            colourFg = color::Fg(color::Black),
            resetFg = color::Fg(color::Reset),
            lines = (0..(menu_width - 11) / 2).map(|_| "─").collect::<String>()
        ),
        format!(
            "{}{}",
            cursor::Goto(x_offset, y_offset + 1),
            (0..menu_width).map(|_| " ").collect::<String>()
        ),
        format!(
            "{goto}{colorFg}{offset}select with <enter>{offset}{resetFg}",
            goto = cursor::Goto(x_offset, y_offset + 2),
            colorFg = color::Fg(color::Rgb(196, 194, 194)),
            resetFg = color::Fg(color::Reset),
            offset = (0..(menu_width - 19) / 2).map(|_| " ").collect::<String>()
        ),
        format!(
            "{goto}{colorFg}{offset}or with <1-{max_choice}>{offset}{resetFg}",
            goto = cursor::Goto(x_offset, y_offset + 3),
            colorFg = color::Fg(color::Rgb(196, 194, 194)),
            resetFg = color::Fg(color::Reset),
            offset = (0..(menu_width - 13) / 2).map(|_| " ").collect::<String>(),
            max_choice = choices.len()
        ),
        format!(
            "{}{}",
            cursor::Goto(x_offset, y_offset + 4),
            (0..menu_width).map(|_| " ").collect::<String>()
        ),
        format!(
            "{}{}",
            cursor::Goto(x_offset, y_offset + 6 + choices.len() as u16),
            (0..menu_width).map(|_| " ").collect::<String>()
        ),
        format!(
            "{}{} {}{}{}",
            cursor::Goto(x_offset, y_offset + 7 + choices.len() as u16),
            color::Bg(color::Rgb(158, 187, 211)),
            color::Bg(color::Rgb(112, 141, 194)),
            (1..menu_width).map(|_| " ").collect::<String>(),
            color::Bg(color::LightBlue)
        ),
    ];

    for i in 0..choices.len() as u16 {
        let mut y = i;
        if i == choices.len() as u16 - 1 {
            lines.push(format!(
                "{}{}",
                cursor::Goto(x_offset, y_offset + 5 + y),
                (0..menu_width).map(|_| " ").collect::<String>()
            ));
            y += 1;
        }

        let choice_text = format!("{}- {}", i + 1, choices[i as usize]);
        if i as i8 != selected {
            lines.push(format!(
                "{goto}{colorFg}{offset}{choice}{offset}{resetFg}",
                goto = cursor::Goto(x_offset, y_offset + 5 + y),
                colorFg = color::Fg(color::Black),
                resetFg = color::Fg(color::Reset),
                choice = choice_text,
                offset = (0..(menu_width - choice_text.len() as u16) / 2)
                    .map(|_| " ")
                    .collect::<String>()
            ))
        } else {
            lines.push(format!(
                "{goto}{colorFg}{offset}{colorBg}{choice}{resetBg}{offset}{resetFg}",
                goto = cursor::Goto(x_offset, y_offset + 5 + y),
                colorFg = color::Fg(color::Black),
                colorBg = color::Bg(color::LightRed),
                resetFg = color::Fg(color::Reset),
                resetBg = color::Bg(color::LightBlue),
                choice = choice_text,
                offset = (0..(menu_width - choice_text.len() as u16) / 2)
                    .map(|_| " ")
                    .collect::<String>()
            ))
        }
    }

    for line in lines.iter() {
        // let new_line = line;
        let new_line = if lines.iter().position(|n| n == line).unwrap() as u16 != 0 {
            format!(
                "{}{} {}",
                line,
                color::Bg(color::Rgb(112, 141, 194)),
                color::Bg(color::LightBlue)
            )
        } else {
            line.to_string()
        };

        write!(stdout, "{}", new_line).unwrap();
    }

    stdout.flush().unwrap();
}

fn menu() {
    let mut stdout = stdout().lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(stdout, "{}", cursor::Hide).unwrap();
    stdout.flush().unwrap();

    match timetable::load_timetable("res/Timetable.json") {
        Ok(_) => {}
        Err(e) => {
            write!(
                stdout,
                "{}{}{}{}{}",
                cursor::Goto(1, 1),
                clear::CurrentLine,
                color::Fg(color::Red),
                e,
                color::Fg(color::Reset)
            )
            .unwrap();

            process::exit(1);
        }
    }

    let mut selected: i8 = 0;
    let choices: Vec<&str> = vec!["View timetable", "Edit timetable", "Exit"];

    flood_screen((158, 187, 211));
    draw_menu(choices.clone(), selected);

    loop {
        let char_buff = stdin.next(); // input buffer
        if char_buff.is_some() {
            // if there is a value in the buffer

            match char_buff.unwrap() {
                // decode pressed key
                Ok(b'q') => exit_app(),
                Ok(66) => {
                    // down
                    selected += 1;
                    if selected >= choices.len() as i8 {
                        selected = 0;
                    }
                }
                Ok(65) => {
                    // up
                    selected -= 1;
                    if selected < 0 {
                        selected = choices.len() as i8 - 1;
                    }
                }
                Ok(13) => {
                    match choices[selected as usize] {
                        "View timetable" => display_timetable(&mut stdin),
                        "Exit" => exit_app(),
                        _ => {}
                    }

                    flood_screen((158, 187, 211));
                    stdout.flush().unwrap();
                }
                Ok(k) => {
                    let choice = if k - 47 <= choices.len() as u8 && k != 48 {
                        choices[(k - 49) as usize]
                    } else {
                        " "
                    };

                    match choice {
                        "View timetable" => display_timetable(&mut stdin),
                        "Exit" => exit_app(),
                        _ => {}
                    }
                }
                Err(e) => {
                    write!(
                        stdout,
                        "{}{}{}{}{}",
                        clear::All,
                        cursor::Goto(1, 1),
                        color::Fg(color::Red),
                        e,
                        color::Fg(color::Reset)
                    )
                    .unwrap();
                    stdout.flush().unwrap();
                    process::exit(1);
                }
            }
            draw_menu(choices.clone(), selected);
        }

        thread::sleep(Duration::from_millis(1000 / 144));
        stdout.flush().unwrap();
    }
}

fn draw_timetable(current_lesson: (i32, i32), is_a_week: bool) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let timetable_data = timetable::load_timetable("res/Timetable.json").unwrap();
    let mut lines: [[String; 7]; 6] = Default::default();

    let week_day = if is_a_week {
        timetable_data.a
    } else {
        timetable_data.b
    };

    for day_i in 0..5 {
        let day = match day_i {
            0 => week_day.clone().mon,
            1 => week_day.clone().tue,
            2 => week_day.clone().wed,
            3 => week_day.clone().thu,
            4 => week_day.clone().fri,
            _ => week_day.clone().mon,
        };

        let day_lines = timetable::draw_day(day);

        for lesson_i in 0..6 {
            let lesson_lines = day_lines[lesson_i].clone();

            let bg_color = if day_i == current_lesson.0 {
                if lesson_i as i32 == current_lesson.1 {
                    color::Bg(color::Rgb(101, 133, 88))
                } else {
                    color::Bg(color::Rgb(163, 219, 138))
                }
            } else {
                color::Bg(color::Rgb(130, 168, 159))
            };

            for i in 0..7 {
                lines[lesson_i][i] += format!("{}{}", bg_color, lesson_lines[i],).as_str();
            }
        }
    }

    let (width, height) = terminal_size().unwrap();
    let (menu_width, menu_height) = ((24 * 5) + 4, (7 * 6) + 4);

    let mut x_offset = 0;
    let mut y_offset = 0;

    if !(menu_width > width || menu_height > height) {
        x_offset = (width - menu_width) / 2;
        y_offset = (height - menu_height) / 2;
    }

    write!(
        stdout,
        "{goto1}{colorFg}{colorBg}{line2}{goto2}{line1}{shadowBg} {colorBg}",
        goto1 = cursor::Goto(x_offset, y_offset),
        goto2 = cursor::Goto(x_offset, y_offset + 1),
        colorBg = color::Bg(color::Rgb(130, 168, 159)),
        shadowBg = color::Bg(color::Rgb(112, 141, 194)),
        colorFg = color::Fg(color::Black),
        line1 = (0..menu_width).map(|_| " ").collect::<String>(),
        line2 = format!("{:─^124}", "┤Timetable├")
    )
    .unwrap();

    let mut y = y_offset + 2;
    for lesson in lines {
        for i in 0..7 {
            write!(
                stdout,
                "{goto}{colorBg}  {lesson}  {shadowBg} {colorBg}",
                goto = cursor::Goto(x_offset, y),
                colorBg = color::Bg(color::Rgb(130, 168, 159)),
                shadowBg = color::Bg(color::Rgb(112, 131, 194)),
                lesson = lesson[i],
            )
            .unwrap();
            y += 1;
        }
    }
    write!(
        stdout,
        "{goto1}{colorBg}{line1}{shadowBg} {colorBg}{goto2}{line2}{shadowBg} {goto3}{shadowLine}",
        goto1 = cursor::Goto(x_offset, y_offset + (7 * 6) + 2),
        goto2 = cursor::Goto(x_offset, y_offset + (7 * 6) + 3),
        goto3 = cursor::Goto(x_offset + 1, y_offset + (7 * 6) + 4),
        colorBg = color::Bg(color::Rgb(130, 168, 159)),
        shadowBg = color::Bg(color::Rgb(112, 141, 194)),
        line1 = (0..menu_width).map(|_| " ").collect::<String>(),
        line2 = (0..menu_width).map(|_| " ").collect::<String>(),
        shadowLine = (0..menu_width).map(|_| " ").collect::<String>()
    )
    .unwrap();

    stdout.flush().unwrap();
}

fn display_timetable(stdin: &mut std::io::Bytes<termion::AsyncReader>) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    flood_screen((158, 187, 211));

    let current_time = chrono::offset::Local::now();
    let a_week = timetable::load_last_aweek("res/Date").unwrap();

    let is_a_week = if timetable::num_of_weeks_since(a_week, current_time.date_naive()) % 2 == 0 {
        true
    } else {
        false
    };

    let week_day = match current_time.date_naive().weekday() {
        chrono::Weekday::Mon => 0,
        chrono::Weekday::Tue => 1,
        chrono::Weekday::Wed => 2,
        chrono::Weekday::Thu => 3,
        chrono::Weekday::Fri => 4,
        chrono::Weekday::Sat => 5,
        chrono::Weekday::Sun => 6,
    };

    let current_lesson = (
        week_day,
        timetable::get_period_from_time(current_time.time()) as i32,
    );

    draw_timetable(current_lesson, is_a_week);

    loop {
        stdout.flush().unwrap();
        let char_buff = stdin.next();

        if char_buff.is_some() {
            draw_timetable(current_lesson, is_a_week);

            match char_buff.unwrap() {
                Ok(b'q') => return,
                Ok(_) => {}
                Err(e) => {
                    write!(
                        stdout,
                        "{}{}{}{}{}",
                        clear::All,
                        cursor::Goto(1, 1),
                        color::Fg(color::Red),
                        e,
                        color::Fg(color::Reset)
                    )
                    .unwrap();
                    stdout.flush().unwrap();
                    process::exit(1);
                }
            }
        }

        thread::sleep(Duration::from_millis(1000 / 60));
    }
}

fn main() {
    menu()
}
