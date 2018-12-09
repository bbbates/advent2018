//extern crate hyper;
//
//use std::io::{self, Write};
//use hyper::Client;
//use hyper::rt::{self, Future, Stream};
//

use std::result::Result::Ok;
use std::num::ParseIntError;

fn split_frequencies(freqs: &String) -> Vec<&str> {
    freqs.split('\n').collect()
}

fn to_i(freq: &str) -> Result<i16, ParseIntError> {
//    i16::from_str(freq.trim())
    Ok(0)
}

fn apply_freq_changes(starting_freq: i16, freqs: Vec<i16>) -> i16 {
    freqs.iter().fold(starting_freq, |acc, f| acc + f)
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
