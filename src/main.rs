use std::{error::Error, fs::read_to_string, time::Instant};

use serde::Deserialize;

#[derive(Deserialize)]
struct Case {
    name: String,
    input: String,
    expected: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .expect("usage: harness <cases.json>");
    let contents = read_to_string(&path)?;
    let cases: Vec<Case> = serde_json::from_str::<Vec<Case>>(&contents)?;
    for case in &cases {
        let time = Instant::now();
        let got = solve(&case.input);
        if got == case.expected {
            println!(
                "{} PASS! {}us elapsed.",
                case.name,
                time.elapsed().as_micros()
            );
        } else {
            println!(
                "FAIL! {} != {}. {}us elapsed.",
                got,
                case.expected,
                time.elapsed().as_micros()
            )
        }
    }
    Ok(())
}

fn solve(input: &str) -> String {
    match input {
        "80" => "http".into(),
        "22" => "ssh".into(),
        _ => "unknown".into(),
    }
}
