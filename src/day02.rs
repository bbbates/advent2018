use std::collections::HashMap;

fn count_repeats(input: &str) -> (i32, i32) {
    let mut char_counts: HashMap<char,i8> = HashMap::new();
    for c in input.chars() {
        let v = char_counts.get(&c).unwrap_or(&0) + 1;
        char_counts.insert(c, v);
    }

    let has_twos = char_counts.values().any(|v| *v == 2);
    let has_threes = char_counts.values().any(|v| *v == 3);

    (has_twos as i32, has_threes as i32)
}


#[cfg(test)]
mod count_repeats_test {
    use super::*;

    #[test]
    fn all_unique_letters_should_give_zero_result() {
        assert_eq!(count_repeats("abcdefg"), (0, 0))
    }

    #[test]
    fn single_letter_repeated_twice_returns_single_result() {
        assert_eq!(count_repeats("abbcde"), (1, 0))
    }

    #[test]
    fn single_letter_repeated_three_times_returns_single_result() {
        assert_eq!(count_repeats("abcccd"), (0, 1))
    }

    #[test]
    fn letters_repeated_two_and_three_times_returns_dual_result() {
        assert_eq!(count_repeats("bababc"), (1, 1))
    }

    #[test]
    fn multiple_letters_appear_two_times_but_only_counts_once() {
        assert_eq!(count_repeats("aabcdd"), (1, 0))
    }

    #[test]
    fn multiple_letters_appear_three_times_but_only_counts_once() {
        assert_eq!(count_repeats("ababab"), (0, 1))
    }
}

fn calc_checksum(lines: &Vec<&str>) -> i32 {
    let (twos, threes) = lines.into_iter()
        .map(|s| count_repeats(s))
        .fold((0,0), |(acc_twos, acc_threes), (twos, threes)| {
            (acc_twos + twos, acc_threes + threes)
        });
    (twos * threes) as i32
}

#[cfg(test)]
mod calc_checksum_tests {
    use super::*;

    #[test]
    fn no_lines_should_be_zero_checksum() {
        assert_eq!(calc_checksum(&vec!{}), 0)
    }

    #[test]
    fn single_line_no_two_or_threes_checksum_should_be_zero() {
        assert_eq!(calc_checksum(&vec!{"abcde"}), 0)
    }

    #[test]
    fn single_line_with_one_two_checksum_should_be_zero() {
        assert_eq!(calc_checksum(&vec!{"aabcde"}), 0)
    }



    #[test]
    fn problem_example_test() {
        assert_eq!(calc_checksum(&vec!{"abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"}), 12)
    }

}

fn split_input_lines(input: &String) -> Vec<&str> {
    input.split("\n").map(str::trim).filter(|s| !s.is_empty()).collect()
}

pub fn solve_part_one(input: &String) -> String {
    let lines: Vec<&str> = split_input_lines(input);
    format!("{}", calc_checksum(&lines))
}



pub fn solve_part_two(input: &String) -> String {
    let lines: &Vec<&str> = &split_input_lines(input);

    let default_sim = &String::new();
    let mut most_similar = String::new();

    // terrible O(n^2 * l) complexity algo here
    for s in lines {
        for oth in lines {
            if s != oth {
                let mut sim = default_sim.clone();

                for i in 0..s.len() {
                    if s.get(i..i+1).unwrap() == oth.get(i..i+1).unwrap() {
                        sim.push_str(s.get(i..i+1).unwrap());
                    }
                }
//                println!("{}:{} -> {:?}", s, oth, sim);
                if most_similar.len() < sim.len() {
                    most_similar.truncate(0);
                    most_similar.push_str(sim.clone().as_str())
                }
            }
        }
    }

    most_similar
}


#[cfg(test)]
mod solve_part_two_tests {
    use super::*;

    #[test]
    fn acceptance_test() {
        assert_eq!(solve_part_two(&String::from("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz\n")), "fgij")
    }
}
