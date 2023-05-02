use horrorshow::{box_html, helper::doctype, html, owned_html, Render, RenderBox, RenderOnce};
use serde::Deserialize;
use std::{fs::File, io::Write};

#[derive(Debug, Deserialize, Clone)]
struct Meetup {
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

struct Meetups {
    monday: Vec<Meetup>,
    tuesday: Vec<Meetup>,
    wednesday: Vec<Meetup>,
    thursday: Vec<Meetup>,
    friday: Vec<Meetup>,
    saturday: Vec<Meetup>,
    sunday: Vec<Meetup>,
}

fn read_meetups() -> Meetups {
    let meetups_raw = File::open("./data/meetups.csv").unwrap();
    let mut meetups_csv = csv::Reader::from_reader(meetups_raw);
    let mut meetups: Meetups = Meetups {
        monday: Vec::new(),
        tuesday: Vec::new(),
        wednesday: Vec::new(),
        thursday: Vec::new(),
        friday: Vec::new(),
        saturday: Vec::new(),
        sunday: Vec::new(),
    };

    for result in meetups_csv.deserialize() {
        let meetup: Meetup = result.unwrap();

        match meetup.day.clone() {
            day if day.contains("Monday") => meetups.monday.push(meetup),
            day if day.contains("Tuesday") => meetups.tuesday.push(meetup),
            day if day.contains("Wednesday") => meetups.wednesday.push(meetup),
            day if day.contains("Thursday") => meetups.thursday.push(meetup),
            day if day.contains("Friday") => meetups.friday.push(meetup),
            day if day.contains("Saturday") => meetups.saturday.push(meetup),
            day if day.contains("Sunday") => meetups.sunday.push(meetup),
            _ => (),
        }
    }

    return meetups;
}

fn render_head() -> impl Render {
    return owned_html! {
        head {
            title: "Austin Running";
            meta(name="viewport", content="width=device-width, initial-scale=1") {}
            link(rel="stylesheet", href="./styles.css") {}
        }
    };
}

fn render_day(day: String, meetups: Vec<Meetup>) -> Box<dyn RenderBox> {
    box_html! {
        h2(class="font-bold text-3xl mb-8 align-bottom"): day;
        ol(class="mb-16") {
            @ for meetup in meetups {
                li(class="grid grid-cols-3 sm:grid-cols-6 gap-4 mb-4") {
                    div(class="w-24 flex-none col-span-1") : meetup.time;
                    div(class="col-span-2 sm:col-span-5") {
                        div(class="mb-2 block") : meetup.group;
                        div(class="leading-6 mb-2") :meetup.description;
                        div(class="leading-6 mb-2") : meetup.location;
                    }
                }
            }
        }
    }
}

fn main() {
    let meetups = read_meetups();

    let tree = html! {
        :doctype::HTML;
        html {
            :&render_head();
            body {
                main(class="container max-w-3xl mx-auto px-4 pt-8") {
                    :render_day("Monday".to_string(), meetups.monday.clone());
                    :render_day("Tuesday".to_string(), meetups.tuesday.clone());
                    :render_day("Wednesday".to_string(), meetups.wednesday.clone());
                    :render_day("Thursday".to_string(), meetups.thursday.clone());
                    :render_day("Friday".to_string(), meetups.friday.clone());
                    :render_day("Saturday".to_string(), meetups.saturday.clone());
                    :render_day("Sunday".to_string(), meetups.sunday.clone());
                }
            }
        }
    };

    let html = format!("{}", tree);

    let mut file = File::create("./build/index.html").unwrap();
    file.write_all(html.as_bytes()).unwrap();
}
