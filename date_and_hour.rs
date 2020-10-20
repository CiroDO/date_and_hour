use std::io::{ stdin, stdout, Write };
use std::{ thread, time };
use std::process::Command;
const TITLE: &str = "Date and hour app";

/*
 * Este é um app data e hora. Ele deve inicializar o
 * tempo lendo as horas, o dia, mês e ano. Por fim, o
 * tempo deve ser mostrado passando, atualizando
 * as variáveis passadas.
 */

 /*
  * This is a date and hour app. It should assign
  * the hours, year, month and day. Finally, the
  * time should be printed starting on the inputted
  * starting point, and pass seconds by.
  */

fn main() {
    clear_terminal();

    let mut seconds: i32 = 0;
    read_seconds(&mut seconds);
    let mut minutes: i32 = 0;
    read_minutes(&mut minutes);
    let mut hours: i32 = 0;
    read_hours(&mut hours);

    let mut year: i32 = 1;
    read_year(&mut year);
    let leap_year: bool = is_leap(year);

    let mut month: i32 = 1;
    read_month(&mut month);
    let maximum_days: i32 = max_days_in(month, leap_year);

    let mut day: i32 = 1;
    read_day(&mut day, maximum_days);

    let mut app_duration: i64 = 0;
    read_duration(&mut app_duration);
    let mut initial_date_and_time: [i32; 6] = [day, month, year, hours, minutes, seconds];
    init_clock(app_duration, &mut initial_date_and_time);

    print!("\n\nProgram finished.\n");
    print!("See you! c:\n");
}

fn clear_terminal() {
    let mut call = if cfg!(target_os = "windows") {
        Command::new("cls")
    } else { Command::new("clear") };
    call.status().expect("syscall!");
}

fn print_app_header() {
    print!("\n\t{}\n\n", TITLE);
}

fn read_i32_variable(time: &mut i32) {
    print!(">>> "); stdout().flush().expect("write!");

    let mut time_as_string = String::new();
    stdin().read_line(&mut time_as_string).expect("read!");
    *time = time_as_string.trim().parse::<i32>().unwrap_or(-1);
    time_as_string.clear();
}

fn read_i64_variable(time: &mut i64) {
    print!(">>> "); stdout().flush().expect("write!");

    let mut time_as_string = String::new();
    stdin().read_line(&mut time_as_string).expect("read!");
    *time = time_as_string.trim().parse::<i64>().unwrap_or(-1);
    time_as_string.clear();
}

fn read_seconds(seconds: &mut i32) {
    loop {
        print_app_header();
        print!("\nInitialize the seconds:\n");
        read_i32_variable(seconds);
        clear_terminal();
        if given_seconds_are_valid(*seconds) {
            break;
        } else {print!("Invalid secons!");}
    }
}

fn given_seconds_are_valid(seconds: i32) -> bool {
    if seconds >= 0 && seconds < 60 { true }  else { false }
}

fn read_minutes(minutes: &mut i32) {
    loop {
        print_app_header();
        print!("\nInitialize the minutes:\n");
        read_i32_variable(minutes);
        clear_terminal();
        if given_minutes_are_valid(*minutes) {
            break;
        } else {print!("Invalid minutes!");}
    }
}

fn given_minutes_are_valid(minutes: i32) -> bool {
    if minutes >= 0 && minutes < 60 { true } else { false }
}

fn read_hours(hours: &mut i32) {
    loop {
        print_app_header();
        print!("\nInitialize the hours:\n");
        read_i32_variable(hours);
        clear_terminal();
        if given_hours_are_valid(*hours) {
            break;
        } else {print!("Invalid hours!");}
    }
}

fn given_hours_are_valid(hours: i32) -> bool {
    if hours >= 0 && hours < 24 { true } else { false }
}

fn read_year(year: &mut i32) {
    loop {
        print_app_header();
        print!("\nInitialize the year:\n");
        read_i32_variable(year);
        clear_terminal();
        if given_year_is_valid(*year) {
            break;
        } else {print!("Invalid year!");}
    }
}

fn given_year_is_valid(year: i32) -> bool {
    if year >= 1 { true } else { false }
}

fn is_leap(year: i32) -> bool {
    if year < 100 { (year % 4) == 0 }
    else { ((year % 4) == 0) || (((year % 100) == 0) && ((year % 400) == 0)) }
}

fn read_month(month: &mut i32) {
    loop {
        print_app_header();
        print!("\nInitialize the month:\n");
        read_i32_variable(month);
        clear_terminal();
        if given_month_is_valid(*month) {
            break;
        } else {print!("Invalid month!");}
    }
}

fn given_month_is_valid(month: i32) -> bool {
    if month >= 1 && month <= 12 { true } else { false }
}

fn read_day(day: &mut i32, maximum_days: i32) {
    loop {
        print_app_header();
        print!("\nInitialize the day:\n");
        read_i32_variable(day);
        clear_terminal();
        if *day <= maximum_days {
            break;
        } else {print!("Invalid day!");}
    }
}

fn max_days_in(month: i32, is_leap: bool) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {if is_leap { 29 } else { 28 }}
        _ => {print!("The given month isn't valid!"); -1}
    }
}

fn read_duration(time: &mut i64) {
    loop {
        print_app_header();
        print!("\nIn how many seconds should this app run?\n");
        print!("In case of zero, CTRL+C might stop it.\n");
        read_i64_variable(time);
        clear_terminal();
        if *time >= 0 {
            break;
        } else {print!("Give a valid time!");}
    }
}

fn init_clock(duration: i64, initial_time: &mut [i32; 6]) {
    if duration < 1 {
        nonstop_clock(initial_time);
    } else {
        clock_that_lasts(duration, initial_time);
    }
}

fn nonstop_clock(initial_time: &mut [i32; 6]) {
    let one_sec = time::Duration::from_millis(1000);
    loop {
        clear_terminal();
        print_app_header();
        print_instant_time(initial_time);
        thread::sleep(one_sec);
    }
}

fn clock_that_lasts(duration: i64, initial_time:  &mut [i32; 6]) {
    let app_duration = time::Duration::from_millis((duration as u64)*1000);
    let one_sec = time::Duration::from_millis(1000);
    let init_time = time::Instant::now();
    while init_time.elapsed() <= app_duration {
        clear_terminal();
        print_app_header();
        print_instant_time(initial_time);
        thread::sleep(one_sec);
    }
}

fn print_instant_time(initial_time: &mut [i32; 6]) {
    let day = initial_time[0];
    let month = initial_time[1];
    let year = initial_time[2];
    let hours = initial_time[3];
    let minutes = initial_time[4];
    let seconds = initial_time[5];
    let leap_year: bool = is_leap(year);

    if month < 10 { print!("0{}:", month); }
        else { print!("{}:", month); }
    if day < 10 { print!("0{}:", day); }
        else { print!("{}:", day); }
    print!("{} - ", year);

    if hours < 10 { print!("0{}:", hours); }
        else { print!("{}:", hours); }
    if minutes < 10 { print!("0{}:", minutes); }
        else { print!("{}:", minutes); }
    if seconds < 10 { print!("0{}", seconds); }
        else { print!("{}", seconds); }
    stdout().flush().expect("write!");

    initial_time[5] = if seconds < 59 { seconds + 1 } else {
        initial_time[4] = if minutes < 59 { minutes + 1 } else {
            initial_time[3] = if hours < 23 { hours + 1 } else {
                initial_time[0] = if day < max_days_in(month, leap_year) { day + 1 } else {
                    initial_time[1] = if month < 12 { month + 1 } else {
                        initial_time[2] += 1;
                    1 };
                1 };
            0 };
        0 };
    0 };
}
