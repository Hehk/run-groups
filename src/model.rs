use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize, Clone)]
struct SpreadsheetGroup {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Instagram")]
    instagram: String,
    #[serde(rename = "Facebook")]
    facebook: String,
    #[serde(rename = "Twitter")]
    twitter: String,
    #[serde(rename = "Website")]
    website: String,
    #[serde(rename = "Strava")]
    strava: String,
    #[serde(rename = "Description")]
    description: String,
}

#[derive(Debug, Deserialize, Clone)]
struct SpreadsheetMeetup {
    #[serde(rename = "Running Group")]
    group: String,
    #[serde(rename = "Day of the Week")]
    day: String,
    #[serde(rename = "Time")]
    time: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Location")]
    location: String,
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug, Clone)]
pub enum Time {
    Morning,
    Afternoon,
    Evening,
    ClockTime(i8, i8),
}

pub struct Group {
    pub id: String,
    pub name: String,
    pub instagram: Option<String>,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub strava: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Meetup {
    pub group: String,
    pub group_id: String,
    pub day: Day,
    pub time: Time,
    pub description: Option<String>,
    pub location: Option<String>,
}

fn create_group_id(group: String) -> String {
    group.to_lowercase().replace(" ", "-")
}

fn parse_day(raw_day: String) -> Result<Day, &'static str> {
    let day = match raw_day.to_lowercase().as_str() {
        "monday" => Day::Monday,
        "tuesday" => Day::Tuesday,
        "wednesday" => Day::Wednesday,
        "thursday" => Day::Thursday,
        "friday" => Day::Friday,
        "saturday" => Day::Saturday,
        "sunday" => Day::Sunday,
        _ => return Err("Invalid day"),
    };
    Ok(day)
}

fn parse_time(raw_time: String) -> Result<Time, String> {
    let time = raw_time.to_lowercase();
    if time == "morning" {
        return Ok(Time::Morning);
    }
    if time == "afternoon" {
        return Ok(Time::Afternoon);
    }
    if time == "evening" {
        return Ok(Time::Evening);
    }

    let is_pm = if time.contains("pm") { true } else { false };

    let mut parts = time[..5].split(":");
    let hour = parts.next();
    let minute = parts.next();
    match (hour, minute) {
        (Some(hour), Some(minute)) => {
            let hour = match i8::from_str(hour.trim()) {
                Ok(it) => it,
                Err(_) => return Err("Invalid hour".to_string()),
            };
            let minute = match i8::from_str(minute.trim()) {
                Ok(it) => it,
                Err(_) => return Err(format!("Invalid minute: {} {}", minute, raw_time)),
            };
            if is_pm {
                return Ok(Time::ClockTime(hour + 12, minute));
            }
            return Ok(Time::ClockTime(hour, minute));
        }
        _ => Err("Invalid time".to_string()),
    }
}

pub fn read_meetups(csv: String) -> Result<Vec<Meetup>, String> {
    let mut csv_reader = csv::Reader::from_reader(csv.as_bytes());
    let raw_meetups = csv_reader
        .deserialize()
        .into_iter()
        .map(|result| result.unwrap())
        .map(|m: SpreadsheetMeetup| {
            let time = parse_time(m.time)?;
            let day = parse_day(m.day)?;
            let group = m.group.clone();
            let group_id = create_group_id(m.group.clone());
            Ok(Meetup {
                group,
                group_id,
                day,
                time,
                description: if m.description.trim() == "" {
                    None
                } else {
                    Some(m.description)
                },
                location: if m.location.trim() == "" {
                    None
                } else {
                    Some(m.location)
                },
            })
        })
        .collect::<Vec<Result<Meetup, String>>>();

    let mut meetups = Vec::new();
    for raw_meetup in raw_meetups {
        match raw_meetup {
            Ok(meetup) => meetups.push(meetup),
            Err(err) => return Err(err),
        }
    }
    Ok(meetups)
}

pub fn none_if_empty(s: String) -> Option<String> {
    if s.trim() == "" {
        None
    } else {
        Some(s)
    }
}

pub fn read_groups(csv: String) -> Result<Vec<Group>, String> {
    let mut csv_reader = csv::Reader::from_reader(csv.as_bytes());
    let mut groups = Vec::new();
    for raw_group in csv_reader.deserialize() {
        let group: SpreadsheetGroup = match raw_group {
            Ok(it) => it,
            Err(err) => return Err(format!("Error reading group: {}", err)),
        };
        let group_id = create_group_id(group.name.clone());
        groups.push(Group {
            name: group.name,
            id: group_id,
            instagram: none_if_empty(group.instagram),
            facebook: none_if_empty(group.facebook),
            twitter: none_if_empty(group.twitter),
            website: none_if_empty(group.website),
            strava: none_if_empty(group.strava),
            description: none_if_empty(group.description),
        });
    }

    Ok(groups)
}
