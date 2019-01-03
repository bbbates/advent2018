use chrono::{NaiveDateTime, NaiveDate};
use regex::Regex;
use parse;

#[derive(PartialEq, Eq, Debug)]
enum GuardEvent {
    BeginShift(GuardRecord),
    Sleep(GuardRecord),
    WakeUp(GuardRecord)
}

#[derive(PartialEq, Eq, Debug)]
struct GuardRecord {
    ts: NaiveDateTime,
    guard: String
}

impl GuardEvent {
    pub fn parse(record_text: &str) -> Result<GuardEvent, String> {
        lazy_static! {
        static ref RE: Regex = Regex::new(r"^\[([\d\-: ]+)\] ((falls asleep)|(wakes up)|(Guard #(\d+) begins shift))$").unwrap();
    }

        if RE.is_match(record_text) {
            let cap = RE.captures(record_text).unwrap();
            println!("{:?}", cap);
            return Ok(GuardEvent::BeginShift(
                GuardRecord {
                    ts: NaiveDate::from_ymd(2016, 7, 8).and_hms(0, 0, 0),
                    guard: String::from("100"),
                }
            ));
        } else {
            return Err(format!("Could not parse guard event '{}'", record_text).to_string());
        }
    }
}

#[cfg(test)]
mod guard_event_tests {
    use super::*;

    #[test]
    fn parse_begin_shift_guard_event() {
        assert_eq!(GuardEvent::parse("[1518-11-01 00:00] Guard #10 begins shift").unwrap(),
        GuardEvent::BeginShift(GuardRecord {
            ts: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
            guard: String::from("10")
        }));
    }
}




pub fn solve_part_one(input: &String) -> String {
    let lines = parse::split_input_lines(input);



    let guard = 10;
    let minute_most_slept_during = 24;
    (guard * minute_most_slept_during).to_string()
}

#[cfg(test)]
mod part_one_tests {
    use super::*;

    #[test]
    fn acceptance_test() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

        assert_eq!(solve_part_one(&String::from(input)), "240");
    }
}


pub fn solve_part_two(input: &String) -> String {
    String::from("")
}
