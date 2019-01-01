use regex::Regex;
use std::str::FromStr;
use std::collections::HashSet;

fn split_input_lines(input: &String) -> Vec<&str> {
    input.split("\n").map(str::trim).filter(|s| !s.is_empty()).collect()
}

#[derive(PartialEq, Eq, Debug)]
struct RectangleDescriptor {
    id: String,
    pos_left: usize,
    pos_top: usize,
    size_x: usize,
    size_y: usize
}

impl RectangleDescriptor {
    fn squares(&self) -> HashSet<(usize,usize)> {
        let mut expected_squares: HashSet<(usize,usize)> = HashSet::new();
        for r in self.pos_top .. (self.pos_top + self.size_y) {
            for c in self.pos_left .. (self.pos_left + self.size_x) {
                expected_squares.insert((r, c));
            }
        }
        expected_squares
    }
}

#[cfg(test)]
mod squares_for_rectangle_tests {
    use super::*;

    #[test]
    fn squares_for_examples_test() {
        let example_one_rect = RectangleDescriptor {
            id: String::from("1"),
            pos_left: 1,
            pos_top: 3,
            size_x: 4,
            size_y: 4
        };
        let expected_squares: HashSet<(usize,usize)> =
            [(3, 1), (3, 2), (3, 3), (3, 4),
                (4, 1), (4, 2), (4, 3), (4, 4),
                (5, 1), (5, 2), (5, 3), (5, 4),
                (6, 1), (6, 2), (6, 3), (6, 4)]
                .iter().cloned().collect();
        assert_eq!(example_one_rect.squares(), expected_squares);
    }
}

fn parse_rectangle_descriptor(descriptor: &str) -> Result<RectangleDescriptor, String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }

    if RE.is_match(descriptor) {
        let cap = RE.captures(descriptor).unwrap();
        println!("Parsed {}", descriptor);
        return Ok(RectangleDescriptor {
           id: String::from(cap.get(1).unwrap().as_str()),
           pos_left: usize::from_str(cap.get(2).unwrap().as_str()).unwrap(),
           pos_top: usize::from_str(cap.get(3).unwrap().as_str()).unwrap(),
           size_x: usize::from_str(cap.get(4).unwrap().as_str()).unwrap(),
           size_y: usize::from_str(cap.get(5).unwrap().as_str()).unwrap()
        });
    } else {
        return Err(format!("Failed to parse rectangle descriptor: [{}]", descriptor));
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn parse_examples_successful() {
        assert_eq!(parse_rectangle_descriptor("#1 @ 1,3: 4x4").unwrap(), RectangleDescriptor {
            id: String::from("1"),
            pos_left: 1,
            pos_top: 3,
            size_x: 4,
            size_y: 4
        });
        assert_eq!(parse_rectangle_descriptor("#1043 @ 674,568: 12x17").unwrap(), RectangleDescriptor {
            id: String::from("1043"),
            pos_left: 674,
            pos_top: 568,
            size_x: 12,
            size_y: 17
        });
    }


    #[test]
    fn parse_examples_fail() {
        assert_eq!(parse_rectangle_descriptor("rubbish"), Err(String::from("Failed to parse rectangle descriptor: [rubbish]")));
    }
}


pub fn solve_part_one(input: &String) -> String {
    let lines = split_input_lines(input);
    println!("Got line input -> {} lines", lines.len());
    let (_, dupes) = lines.iter()
        .map(|line: &&str| {
            parse_rectangle_descriptor(line).expect(line).squares()
        })
        .fold((HashSet::<(usize,usize)>::new(), HashSet::<(usize,usize)>::new()),
              |(mut all_squares, mut dupes), squares| {
            let new_dupes: HashSet<(usize,usize)> = all_squares.intersection(&squares).map(|s| *s).collect();
            let all_dupes: HashSet<(usize,usize)> = dupes.union(&new_dupes).map(|s| *s).collect();

            let new_all_squares: HashSet<(usize,usize)> = all_squares.union(&squares).map(|s| *s).collect();

            (new_all_squares, all_dupes)
        });

    dupes.len().to_string()
}


#[cfg(test)]
mod part_one_tests {
    use super::*;

    #[test]
    fn acceptance_tests() {
        assert_eq!(solve_part_one(&String::from("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n")), "4")
    }

}
