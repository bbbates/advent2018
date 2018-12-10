extern crate reqwest;

use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::HashSet;

fn split_frequencies(freqs: &String) -> Vec<&str> {
    freqs.split('\n')
        .map(|f| f.trim())
        .collect()
}

fn to_i(freq: &str) -> Result<i32, ParseIntError> {
    i32::from_str(freq.trim())
}

pub fn solve(input: &String) -> String {
    let result: i32  = split_frequencies(&input)
        .iter()
        .filter(|f| !f.is_empty())
        .map(|f| to_i(*f).expect("Could not parse {}"))
        .sum();
    format!("{}", result)
}

pub fn solve_part_two(input: &String) -> String {
    let mut seen: HashSet<i32> = HashSet::new();
    let first_match = split_frequencies(&input)
        .iter()
        .filter(|f| !f.is_empty())
        .map(|f| to_i(*f).expect("Could not parse {}"))
        .cycle()
        .try_fold(0,
                  |acc, f| {
                      if seen.contains(&(f + acc)) {
                          Err(f + acc)
                      } else {
                          seen.insert(f + acc);
                          Ok(f+acc)
                      }
                  });
    match first_match {
        Ok(_) => "Not found!".to_string(),
        Err(x) => x.to_string()
    }

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
mod first_frequency_reaches_twice_tests {
    use super::*;

    #[test]
    fn example_test() {
        assert_eq!(solve_part_two(&String::from("+1\n-2\n+3\n+1")), "2");
        assert_eq!(solve_part_two(&String::from("+3\n+3\n+4\n-2\n-4")), "10");
        assert_eq!(solve_part_two(&String::from("-6\n+3\n+8\n+5\n-6")), "5");
        assert_eq!(solve_part_two(&String::from("+7\n+7\n-2\n-7\n-4")), "14");
    }
}


