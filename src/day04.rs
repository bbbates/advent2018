use chrono::{NaiveDateTime, NaiveDate, Timelike};
use regex::Regex;
use parse;
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;
use std::cmp::Ordering;

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
    message: Message,
    guard: String,
}

impl GuardEvent {
    pub fn parse(message: &Message) -> Result<GuardEvent, String> {
        GuardEvent::parse_for_guard(String::from(""), message)
    }

    pub fn parse_for_guard(guard: String, message: &Message) -> Result<GuardEvent, String> {
        lazy_static! {
        static ref RE: Regex = Regex::new(r"((falls asleep)|(wakes up)|(Guard #(\d+) begins shift))$").unwrap();
    }

        let message_msg = message.message.to_string();

        if RE.is_match(message_msg.as_str()) {
            let cap = RE.captures(message_msg.as_str()).unwrap();

            println!("{:?}", cap);

            let parsed_guard = match cap.get(5) {
                Some(m) => String::from(m.as_str()),
                None => String::new()
            };

            let guard_id = if parsed_guard.is_empty() { guard } else { parsed_guard };
            let guard_record = GuardRecord {
                message: message.clone(),
                guard: guard_id,
            };
            let event_desc = cap.get(1).unwrap().as_str();
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
            return Err(format!("Could not parse guard event '{}'", message_msg).to_string());
        }
    }
}

#[cfg(test)]
mod guard_event_tests {
    use super::*;

    #[test]
    fn parse_begin_shift_guard_event() {
        let expected_message = Message {
            message: String::from("Guard #10 begins shift"),
            ts: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
        };
        assert_eq!(GuardEvent::parse(&expected_message),
                   Ok(GuardEvent::BeginShift(GuardRecord {
                       message: expected_message,
                       guard: String::from("10"),
                   })));
    }

    #[test]
    fn parse_invalid_event_should_produce_err_result() {
        let invalid_message = Message {
            message: String::from("Gah, not an event!"),
            ts: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
        };
        assert_eq!(GuardEvent::parse(&invalid_message),
                   Err(String::from("Could not parse guard event 'Gah, not an event!'")));
    }


    #[test]
    fn parse_sleep_event() {
        let expected_message = Message {
            message: String::from("falls asleep"),
            ts: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
        };
        assert_eq!(GuardEvent::parse_for_guard(String::from("10"), &expected_message).unwrap(),
                   GuardEvent::Sleep(GuardRecord {
                       message: expected_message,
                       guard: String::from("10"),
                   }));
    }

    #[test]
    fn parse_wakeup_event() {
        let expected_message = Message {
            message: String::from("wakes up"),
            ts: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
        };
        assert_eq!(GuardEvent::parse_for_guard(String::from("10"), &expected_message).unwrap(),
                   GuardEvent::WakeUp(GuardRecord {
                       message: expected_message,
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
                sleep_record.push(previous_event.unwrap().record().message.ts.minute()..gr.message.ts.minute());
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
            (range.end - 1) - range.start
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
            let ctr = minute_counts.entry(m).or_insert(0);
            *ctr += 1;
        }
    }
    *minute_counts.iter().max_by_key(|c| { (*c).1 }).unwrap().0
}

#[derive(PartialEq, PartialOrd, Eq, Clone, Debug)]
struct Message {
    ts: NaiveDateTime,
    message: String,
}

impl Ord for Message {
    fn cmp(&self, other: &Message) -> Ordering {
        self.ts.cmp(&other.ts)
    }
}

impl Message {
    pub fn parse(record_text: &str) -> Result<Message, String> {
        lazy_static! {
        static ref RE: Regex = Regex::new(r"^\[([\d\-: ]+)\] (.+)$").unwrap();
    }

        if RE.is_match(record_text) {
            let cap = RE.captures(record_text).unwrap();

            if let Ok(dt) = NaiveDateTime::parse_from_str(cap.get(1).unwrap().as_str(), "%Y-%m-%d %H:%M") {
                Ok(Message {
                    ts: dt,
                    message: String::from(cap.get(2).unwrap().as_str()),
                })
            } else {
                Err(format!("Could not parse message datetime '{}'", record_text).to_string())
            }
        } else {
            return Err(format!("Could not parse message '{}'", record_text).to_string());
        }
    }
}

#[cfg(test)]
mod message_tests {
    use super::*;

    #[test]
    fn parse_valid_message() {
        assert_eq!(Message::parse("[1518-11-01 00:00] Guard #10 begins shift").unwrap(),
                   Message {
                       ts: NaiveDate::from_ymd(1518, 11, 1).and_hms(0, 0, 0),
                       message: String::from("Guard #10 begins shift"),
                   });
    }

    #[test]
    fn parse_invalid_message_should_produce_err_result() {
        assert_eq!(Message::parse("[1518-13-01 00:00] Guard #10 begins shift"),
                   Err(String::from("Could not parse message datetime '[1518-13-01 00:00] Guard #10 begins shift'")));

        assert_eq!(Message::parse("[1518-11-01 00:00]"),
                   Err(String::from("Could not parse message '[1518-11-01 00:00]'")));
    }
}


pub fn solve_part_one(input: &String) -> String {
    let lines = parse::split_input_lines(input);
    let mut messages: Vec<Message> =
        lines.iter().map(|line| {
            Message::parse(line).expect(format!("Could not parse guard event! '{}'", line).as_str())
        }).collect();

    messages.sort();

    println!("{:?}", messages.iter().take(10));
    let guard_events = messages.iter().fold(Vec::<GuardEvent>::new(), |mut events, message| {
        let event = match events.last() {
            None => {
                GuardEvent::parse(message).expect("Could not parse new event")
            }
            Some(ev) => {
                let guard = ev.record().guard.to_string();
                GuardEvent::parse_for_guard(guard, message).expect("Could not parse new event")
            }
        };
        events.push(event);
        events
    });

    let sleep_times = guard_sleep_times(&guard_events);
    let (guard, sleep_ranges) = guard_with_most_times_asleep(&sleep_times);

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
