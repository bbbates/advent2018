/// Split an Advent of Code input into lines, filtering the empty ones.
///
pub fn split_input_lines(input: &String) -> Vec<&str> {
    input.split("\n").map(str::trim).filter(|s| !s.is_empty()).collect()
}
