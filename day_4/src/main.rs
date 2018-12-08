extern crate chrono;
extern crate regex;

use chrono::prelude::*;
use regex::Regex;
use std::io;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Action {
    Wakeup,
    Sleep,
    Start
}
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Event {
    timestamp: DateTime<Utc>,
    guard: i64,
    action: Action
}

fn main() {
    let event_r = Regex::new(r"\[(.+)\]\s+(.+)").unwrap();
    let numbers_r = Regex::new(r"\d+").unwrap();
    let wakes_r = Regex::new(r"wakes up").unwrap();
    let mut logs: Vec<String> = vec![];
    let mut events: Vec<Event> = vec![];
    let mut sleep_times: HashMap<i64, HashMap<u32, i64>> = HashMap::new();

    // Get all the logs
    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                logs.push(input);
            }
            Err(_) => break
        }
    }

    // Sort all the logs - this uses a nice side-effect where timestamps can be
    // ordered as strings
    logs.sort();

    // Build event structs - for nicety
    let mut current_guard = -1;
    for log in &logs {
        let captures = event_r.captures(log).unwrap();
        let mut action = match wakes_r.captures(&captures[2]) {
            Some(_) => Action::Wakeup,
            _ => Action::Sleep,
        };

        if let Some(new_guard) = numbers_r.captures(&captures[2]) {
            current_guard = new_guard[0].parse().unwrap();
            action = Action::Start;
        }

        let event = Event {
            timestamp: Utc
                .datetime_from_str(&captures[1], "%Y-%m-%d %H:%M")
                .unwrap(),
            guard: current_guard,
            action: action
        };

        events.push(event);
    }

    let mut current_guard = -1;
    let mut asleep: Option<DateTime<Utc>> = None;
    for e in events {
        if e.action == Action::Start {
            asleep = None;
            current_guard = e.guard;
        }
        else if e.action == Action::Sleep {
            asleep = Some(e.timestamp);
        }
        else {
            for t in asleep.unwrap().minute()..e.timestamp.minute() {
                *sleep_times
                    .entry(current_guard)
                    .or_insert(HashMap::new())
                    .entry(t)
                    .or_insert(1) += 1;
            }
        }
    };

    let mut count_vec: Vec<(i64, i64)> =
        sleep_times
        .iter()
        .map(|(id, map)|
            (
                id.to_owned(),
                map
                .iter()
                .fold(0, |acc, (_t, c)| acc + c )
            )
        )
        .collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let sleepiest_guard = i64::from(count_vec.first().unwrap().0.to_owned());

    let mut count_vec: Vec<(&u32, &i64)> =
        sleep_times[&sleepiest_guard].iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let sleepiest_minute = i64::from(count_vec.first().unwrap().0.to_owned());

    println!("SLEEPIEST GUARD {:?}", sleepiest_guard);
    println!("SLEEPIEST MINUTE {:?}", sleepiest_minute);
    println!("HASH {:?}", sleepiest_minute * sleepiest_guard);

    let mut sleepiest_minute = 0;
    let mut sleepiest_minute_count = 0;
    let mut sleepiest_guard = 0;
    for (g, d) in sleep_times.iter() {
        for (t, c) in d.iter() {
            if *c >= sleepiest_minute_count {
                sleepiest_minute_count = *c;
                sleepiest_minute = i64::from(*t);
                sleepiest_guard = *g;
            }
        }
    }

    println!("MOST COMMON GUARD {:?}", sleepiest_guard);
    println!("TOTAL SLEEPIEST MINUTE {:?}", sleepiest_minute);
    println!("HASH {:?}", sleepiest_guard * sleepiest_minute);
}
