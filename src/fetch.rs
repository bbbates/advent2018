extern crate reqwest;

use std::error::Error;

pub fn input_for_day(sess: &str, day_input: &str) -> Result<String, Box<Error>> {
    let client = reqwest::Client::new();
    Ok(client.get(format!("https://adventofcode.com/2018/day/{}/input", day_input).as_str())
        .header("Cookie", format!("session={}", sess))
        .send()?
        .text()?)
}
