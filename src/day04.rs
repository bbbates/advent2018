use chrono::{NaiveDateTime, NaiveDate, Timelike};
use regex::Regex;
use parse;
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
enum GuardEvent {
    BeginShift(GuardRecord),
    Sleep(GuardRecord),
    WakeUp(GuardRecord),
}

impl GuardEvent {
    pub fn record(&self) -> &GuardRecord {
        match self {
            GuardEvent::BeginShift(x) => x,
            GuardEvent::Sleep(x) => x,
            GuardEvent::WakeUp(x) => x
        }
    }
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

            if let Ok(dt) = NaiveDateTime::parse_from_str(cap.get(1).unwrap().as_str(), "%Y-%m-%d %H:%M") {
                let parsed_guard = match cap.get(6) {
                    Some(m) => String::from(m.as_str()),
                    None => String::new()
                };
                let guard_id = if parsed_guard.is_empty() { guard } else { parsed_guard };
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

type GuardSleepRanges = HashMap<String, Vec<Range<u32>>>;

fn guard_sleep_times(guard_events: &Vec<GuardEvent>) -> GuardSleepRanges {
    let mut guard_sleep_times = HashMap::<String, Vec<Range<u32>>>::new();

    guard_events.iter().fold(None, |previous_event, event| {
        let sleep_record = guard_sleep_times.entry(event.record().guard.to_string()).or_insert(Vec::<Range<u32>>::new());
        match event {
            GuardEvent::Sleep(_) => Some(event),
            GuardEvent::WakeUp(gr) => {
                sleep_record.push(previous_event.unwrap().record().ts.minute()..gr.ts.minute());
                None
            }
            _ => None
        }
    });

    guard_sleep_times
}

fn guard_with_most_times_asleep(sleep_times: &GuardSleepRanges) -> (String, &Vec<Range<u32>>) {
    let mut sleep_times_flattened = sleep_times.iter().collect::<Vec<(&String, &Vec<Range<u32>>)>>();
    sleep_times_flattened.sort_by_key(|(_, sleep_ranges)| -> u32 {
        (*sleep_ranges).iter().map(|range| {
            (range.end-1) - range.start
        }).sum()
    });
    let (guard, sleep_ranges) = sleep_times_flattened.last().unwrap();
    (guard.to_string(), &sleep_ranges.clone())
}

fn minute_most_asleep(sleep_ranges: &Vec<Range<u32>>) -> u32 {
    let mut minute_counts = HashMap::<u32, u32>::new();
    for range in sleep_ranges.iter() {
        let r: Vec<u32> = range.clone().collect();
        for m in r {
           let ctr =  minute_counts.entry(m).or_insert(0);
            *ctr += 1;
        }
    }
    *minute_counts.iter().max_by_key(|c| { (*c).1 }).unwrap().0
}

#[derive(PartialEq, Eq, Debug)]
struct Message {
    ts: NaiveDateTime,
    message: String,
}

pub fn solve_part_one(input: &String) -> String {
    let lines = parse::split_input_lines(input);
    let (_, mut guard_events) =
        lines.iter().fold((String::new(), Vec::<GuardEvent>::new()), |(current_guard, mut events_acc), line| {
            let event = if current_guard.is_empty() {
                GuardEvent::parse(line)
            } else {
                GuardEvent::parse_for_guard(current_guard, line)
            }.expect(format!("Could not parse guard event! '{}'", line).as_str());
            let next_guard = event.record().guard.to_string();

            events_acc.push(event);

            (next_guard, events_acc)
        });
    // TODO: need to split parser into two phases - one to parse timestamps, then the next one to parse the message

    guard_events.sort_by_key(|k| k.record().ts);
    println!("{:?}", guard_events.iter().take(10));

    let sleep_times = guard_sleep_times(&guard_events);
    let (guard , sleep_ranges) = guard_with_most_times_asleep(&sleep_times);

    let minute_most_slept_during = minute_most_asleep(sleep_ranges);

    (u32::from_str(&guard).unwrap() * minute_most_slept_during).to_string()
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


pub fn solve_part_two(_input: &String) -> String {
    String::from("")
}
