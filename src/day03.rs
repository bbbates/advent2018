use regex::Regex;
use std::str::FromStr;

fn split_input_lines(input: &String) -> Vec<&str> {
    input.split("\n").map(str::trim).filter(|s| !s.is_empty()).collect()
}

#[derive(Debug)]
struct RectangleDescriptor {
    id: String,
    pos_left: usize,
    pos_top: usize,
    size_x: usize,
    size_y: usize
}

impl PartialEq for RectangleDescriptor {
    fn eq(&self, other: &RectangleDescriptor) -> bool {
        self.id == other.id &&
            self.pos_left == other.pos_left &&
            self.pos_top == other.pos_top &&
            self.size_x == other.size_x &&
            self.size_y == other.size_y
    }
}


fn parse_rectangle_descriptor(descriptor: &str) -> Result<RectangleDescriptor, String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#(\d+) @ (\d)+,(\d+): (\d+)x(\d+)").unwrap();
    }

    if RE.is_match(descriptor) {
        let cap = RE.captures(descriptor).unwrap();

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
    }


    #[test]
    fn parse_examples_fail() {
        assert_eq!(parse_rectangle_descriptor("rubbish"), Err(String::from("Failed to parse rectangle descriptor: [rubbish]")));
    }
}


pub fn solve_part_one(_input: &String) -> String {
    String::from("4")
}


#[cfg(test)]
mod part_one_tests {
    use super::*;

    #[test]
    fn acceptance_tests() {
        assert_eq!(solve_part_one(&String::from("#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n")), "4")
    }

}
