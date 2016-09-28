extern crate termion;
extern crate clap;

use std::io::Write;
use std::time::Duration;
use std::{thread, io};

use termion::{cursor, color};
use termion::raw::IntoRawMode;

const PARTIAL_BOXES: [char; 7] = ['▏',
                                  '▎',
                                  '▍',
                                  '▌',
                                  '▋',
                                  '▊',
                                  '▉'];
const FULL_BOX: char = '█';

fn parse_number(opt: Option<&str>) -> u64 {
    use std::error::Error;

    if let Some(numeral) = opt {
        numeral.parse::<u64>().unwrap_or_else(|err| {
            clap::Error::with_description(err.description(), clap::ErrorKind::InvalidValue).exit()
        })
    } else { 0 }
}

fn main() {
    let matches = clap::App::new("rem")
        .setting(clap::AppSettings::ColoredHelp)
        .about("A small program to set reminder and keeping track of the time gone.")
        .version("0.1.0")
        .author("(by Ticki)")
        .arg(clap::Arg::with_name("millis")
             .short("M")
             .long("milli")
             .value_name("NUMBER")
             .multiple(true)
             .empty_values(false)
             .help("Waits NUMBER milliseconds."))
        .arg(clap::Arg::with_name("secs")
             .short("s")
             .long("sec")
             .value_name("NUMBER")
             .multiple(true)
             .empty_values(false)
             .help("Waits NUMBER seconds."))
        .arg(clap::Arg::with_name("mins")
             .short("m")
             .long("min")
             .value_name("NUMBER")
             .multiple(true)
             .empty_values(false)
             .help("Waits NUMBER minutes."))
        .arg(clap::Arg::with_name("hours")
             .short("H")
             .long("hour")
             .value_name("NUMBER")
             .multiple(true)
             .empty_values(false)
             .help("Waits NUMBER hours."))
        .arg(clap::Arg::with_name("bar")
             .short("n")
             .long("len")
             .value_name("NUMBER")
             .default_value("50")
             .multiple(true)
             .empty_values(false)
             .help("Make the progress bar NUMBER long."))
        .get_matches();

    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let sleep = Duration::from_millis(parse_number(matches.value_of("millis")))
        + Duration::from_secs(parse_number(matches.value_of("secs")))
        + Duration::from_secs(60 * parse_number(matches.value_of("mins")))
        + Duration::from_secs(60 * 60 * parse_number(matches.value_of("hours")));
    let bar = parse_number(matches.value_of("bar")) as u32;

    write!(stdout, "{}{}", cursor::Hide, color::Bg(color::LightBlack)).unwrap();
    for _ in 0..bar {
        write!(stdout, " ").unwrap();
    }
    write!(stdout, "{}", cursor::Left(bar as u16)).unwrap();

    let sleep_per_box = sleep / bar;
    let sleep_per_subbox = sleep_per_box / (PARTIAL_BOXES.len() as u32 + 1);

    for _ in 0..bar {
        for i in &PARTIAL_BOXES {
            write!(stdout, "{}{}", i, cursor::Left(1)).unwrap();
            stdout.flush().unwrap();
            thread::sleep(sleep_per_subbox);
        }
        write!(stdout, "{}", FULL_BOX).unwrap();
        stdout.flush().unwrap();
        thread::sleep(sleep_per_subbox);
    }

    write!(stdout, "{}{}\n\r", color::Bg(color::Reset), cursor::Show).unwrap();
}
