extern crate reqwest;

use std::result::Result::Ok;
use std::str::FromStr;
use std::num::ParseIntError;
use std::error::Error;


fn split_frequencies(freqs: &String) -> Vec<&str> {
    freqs.split('\n')
        .map(|f| f.trim())
        .collect()
}

fn to_i(freq: &str) -> Result<i32, ParseIntError> {
    i32::from_str(freq.trim())
}

fn fetch_input(pwd: &str) -> Result<String, Box<Error>> {
    let client = reqwest::Client::new();
    Ok(client.get("https://adventofcode.com/2018/day/1/input")
        .header("Cookie", format!("session={}", pwd))
        .send()?
        .text()?)
}

pub fn solve(pwd: &str) -> String {
    let input = fetch_input(pwd).expect("Could not fetch frequencies!");
    let result: i32  = split_frequencies(&input)
        .iter()
        .filter(|f| !f.is_empty())
        .map(|f| to_i(*f).expect("Could not parse {}"))
        .sum();
    format!("{}", result)
}


#[cfg(test)]
mod split_freqs_tests {
    use super::*;

    #[test]
    fn split_frequencies_for_one_freq() {
        assert_eq!(split_frequencies(&String::from("+100")), ["+100"]);
    }

    #[test]
    fn split_frequencies_for_mult_freqs() {
        assert_eq!(split_frequencies(&String::from("+100\n-100\n+10")),
                   ["+100", "-100", "+10"]);
    }
}

#[cfg(test)]
mod sum_freq_acceptance_tests {
    use super::*;

    #[test]
    fn starting_from_freq_zero() {
        assert_eq!(apply_freq_changes(0, vec![1, -2, 3, 1]), 3);
        assert_eq!(apply_freq_changes(0, vec![1, 1, 1]), 3);
        assert_eq!(apply_freq_changes(0, vec![1, 1, -2]), 0);
        assert_eq!(apply_freq_changes(0, vec![-1, -2, -3]), -6);
    }
}
