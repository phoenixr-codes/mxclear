use std::process;
use std::thread;
use std::time::Duration;

use clap::Arg;
use rand::Rng;

fn cli() -> clap::Command {
    clap::Command::new("mxclear")
        .author("Jonas da Silva")
        .version("0.1.0")
        .about("`$ clear` with fashion")
        .arg(
            Arg::new("style")
                .help("Sets the style froma dotted notation")
                // TODO: add long help
                .long("style")
                .env("MXCLEAR_STYLE")
                .default_value("green"),
        )
        .arg(
            Arg::new("speed")
                .help("Sets the intervall between updating a line in milliseconds")
                .long("speed")
                .env("MXCLEAR_SPEED")
                .default_value("14"),
        )
}

fn main() {
    let mut rng = rand::thread_rng();
    let matches = cli().get_matches();
    let term = console::Term::stdout();
    let style = console::Style::from_dotted_str(&matches.get_one::<String>("style").unwrap());

    term.hide_cursor().unwrap();

    let (rows, columns) = term.size();

    for _ in 0..rows {
        let mut line = String::new();
        for _ in 0..columns {
            let character = match rng.gen_range(0..=2) {
                0 => '0',
                1 => '1',
                2 => ' ',
                _ => unreachable!(),
            };
            line.push(character);
        }
        while !line.trim().is_empty() {
            term.clear_last_lines(1).unwrap();
            term.write_line(&style.apply_to(&line.as_str()).to_string())
                .unwrap();
            line = line.replace('0', " ").replace('1', "0");
            thread::sleep(Duration::from_millis(
                matches
                    .get_one::<String>("speed")
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
            ));
        }
        term.clear_last_lines(1).unwrap();
    }

    term.show_cursor().unwrap();
    
    // really clean the entire terminal afterwards
    if cfg!(target_os = "windows") {
        process::Command::new("cmd")
            .args(["/C", "cls"])
            .spawn()
    } else {
        process::Command::new("bash")
            .args(["-c", "clear"])
            .spawn()
    }.unwrap().wait().unwrap();
}
