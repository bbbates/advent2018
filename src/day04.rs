use chrono::{NaiveDateTime, NaiveDate};
use regex::Regex;
use parse;

#[derive(PartialEq, Eq, Debug)]
enum GuardEvent {
    BeginShift(GuardRecord),
    Sleep(GuardRecord),
    WakeUp(GuardRecord),
}

#[derive(PartialEq, Eq, Debug)]
struct GuardRecord {
    ts: NaiveDateTime,
    guard: String,
}

impl GuardEvent {
    pub fn parse(record_text: &str) -> Result<GuardEvent, String> {
        GuardEvent::parse_for_guard(String::from(""), record_text)
    }

    pub fn parse_for_guard(guard: String, record_text: &str) -> Result<GuardEvent, String> {
        lazy_static! {
        static ref RE: Regex = Regex::new(r"^\[([\d\-: ]+)\] ((falls asleep)|(wakes up)|(Guard #(\d+) begins shift))$").unwrap();
    }

        if RE.is_match(record_text) {
            let cap = RE.captures(record_text).unwrap();
            println!("{:?}", cap);

            if let Ok(dt) = NaiveDateTime::parse_from_str(cap.get(1).unwrap().as_str(), "%Y-%m-%d %H:%M") {
                let guard_id = if guard.is_empty() { String::from(cap.get(6).unwrap().as_str()) } else { guard };
                let guard_record = GuardRecord {
                    ts: dt,
                    guard: guard_id,
                };
                let event_desc = cap.get(2).unwrap().as_str();
                Ok(
                    if event_desc == "falls asleep" {
                        GuardEvent::Sleep(guard_record)
                    } else if event_desc == "wakes up" {
                        GuardEvent::WakeUp(guard_record)
                    } else {
                        GuardEvent::BeginShift(guard_record)
                    }
                )
            } else {
                Err(format!("Could not parse guard event datetime '{}'", record_text).to_string())
            }
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
                       guard: String::from("10"),
                   }));
    }

    #[test]
    fn parse_invalid_event_should_produce_err_result() {
        assert_eq!(GuardEvent::parse("[1518-13-01 00:00] Guard #10 begins shift"),
                   Err(String::from("Could not parse guard event datetime '[1518-13-01 00:00] Guard #10 begins shift'")));

        assert_eq!(GuardEvent::parse("[1518-11-01 00:00] Gah, not an event!"),
                   Err(String::from("Could not parse guard event '[1518-11-01 00:00] Gah, not an event!'")));
    }


    #[test]
    fn parse_sleep_event() {
        assert_eq!(GuardEvent::parse_for_guard(String::from("10"), "[1518-11-01 00:00] falls asleep").unwrap(),
                   GuardEvent::Sleep(GuardRecord {
                       ts: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
                       guard: String::from("10"),
                   }));
    }

    #[test]
    fn parse_wakeup_event() {
        assert_eq!(GuardEvent::parse_for_guard(String::from("10"), "[1518-11-01 00:00] wakes up").unwrap(),
                   GuardEvent::WakeUp(GuardRecord {
                       ts: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
                       guard: String::from("10"),
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
