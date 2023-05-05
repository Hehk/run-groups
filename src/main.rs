use std::{fs::File, io::Write};

use dotenv::dotenv;
use std::collections::HashMap;

use run_groups::model::{Day, Group, Meetup, Time};
use run_groups::sheet;

fn create_head(title: &str) -> String {
    format!(
        "<head>
    <meta charset=\"utf-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
    <link rel=\"stylesheet\" href=\"./styles.css\" as=\"style\">
    <title>{}</title>
</head>",
        title
    )
}

fn render_time(time: &Time) -> String {
    match time {
        Time::Morning => "Morning".to_string(),
        Time::Afternoon => "Afternoon".to_string(),
        Time::Evening => "Evening".to_string(),
        Time::ClockTime(hour, minute) => {
            let hour = *hour;
            let minute = *minute;

            let is_pm = if hour > 12 { true } else { false };
            let hour = if is_pm { hour - 12 } else { hour };
            let minute = if minute < 10 {
                format!("0{}", minute)
            } else {
                minute.to_string()
            };
            let suffix = if is_pm { "PM" } else { "AM" };

            format!("{}:{} {}", hour, minute, suffix)
        }
    }
}

fn render_meetups(meetups: &Vec<&Meetup>) -> String {
    let mut content = String::new();
    let mut meetups = meetups.clone();
    meetups.sort_by(|a, b| {
        let a_time = &a.time;
        let b_time = &b.time;
        fn time_for_enum(time: &Time) -> i16 {
            match time {
                Time::Morning => 6 * 60,
                Time::Afternoon => 12 * 60,
                Time::Evening => 18 * 60,
                Time::ClockTime(hour, minute) => (*hour as i16) * 60 + (*minute as i16),
            }
        }

        (&time_for_enum(a_time)).cmp(&time_for_enum(b_time))
    });

    for meetup in meetups {
        let time = render_time(&meetup.time);
        let group = &meetup.group;
        let group_id = &meetup.group_id;
        let description = match &meetup.description {
            Some(d) => format!(r#"<div>{}</div>"#, d),
            None => "".to_string(),
        };
        let location = match &meetup.location {
            Some(l) => format!(r#"<div>{}</div>"#, l),
            None => "".to_string(),
        };

        let str = format!(
            r#"<li class="grid grid-cols-3 sm:grid-cols-6 gap-4 mb-4">
<div class="w-24 flex-none col-span-1">{time}</div>
<div class="col-span-2 sm:col-span-5 space-y-2">
    <a href="/groups/{group_id}" class="underline decoration-green-400 hover:text-green-400 underline-offset-4">{group}</a>
    {description}
    {location}
</div>
</li>"#
        );
        content.push_str(&str)
    }
    content
}

fn render_day(day: &str, meetups: &Vec<&Meetup>) -> String {
    format!(
        "
<section>
    <h2 class=\"font-bold text-3xl mb-8 align-bottom\">{}</h2>
    <ol class=\"mb-16\">
        {}
    </ol>
</section>",
        day,
        render_meetups(meetups)
    )
}

fn group_meetups(meetups: &Vec<Meetup>) -> HashMap<&Day, Vec<&Meetup>> {
    let mut meetups_by_day: HashMap<&Day, Vec<&Meetup>> = HashMap::new();
    for meetup in meetups {
        let day = &meetup.day;
        match meetups_by_day.get_mut(day) {
            Some(group) => group.push(meetup),
            None => {
                meetups_by_day.insert(day, vec![meetup]);
            }
        };
    }

    return meetups_by_day;
}

fn create_group_content(meetups: &Vec<Group>) -> String {
    let main_content = meetups
        .iter()
        .map(|group| {
            let id = &group.id;
            let name = &group.name;
            fn link (name: &str, url: &Option<String>) -> String {
                match url {
                    Some(u) => format!(r#"<a class="underline decoration-green-400 hover:text-green-400 underline-offset-4 mr-4 leading-8" href="{u}">{name}</a>"#, u = u),
                    None => "".to_string(),
                }
            }

            let description = match &group.description {
                Some(d) => format!(r#"<p class="leading-6 mb-2">{}</p>"#, d),
                None => "".to_string(),
            };
            let website = link("Website", &group.website);
            let facebook = link("Facebook", &group.facebook);
            let twitter = link("Twitter", &group.twitter);
            let instagram = link("Instagram", &group.instagram);
            let strava = link("Strava", &group.strava);


            format!(
        r#"<li class="mb-8">
    <h3 class="text-2xl mb-4 align-bottom" id="{id}">{name}</h3>
    {description}
    <div class="mb-2">
        {website}
        {facebook}
        {twitter}
        {instagram}
        {strava}
    </div>
</li>"#)
        })
        .collect::<Vec<String>>()
        .join("");

    format!(r#"<section>
    <h2 class="font-bold text-3xl mb-8 align-bottom">Groups</h2>
    <ol class="mb-16">
        {main_content}
    </ol>
</section>"#,
        main_content = main_content
    )
}

fn create_meetup_content(meetups: &Vec<Meetup>) -> String {
    let meetups_by_day = group_meetups(meetups);

    let mut content = String::new();

    let monday = meetups_by_day.get(&Day::Monday).unwrap();
    content.push_str(render_day("Monday", monday).as_str());

    let tuesday = meetups_by_day.get(&Day::Tuesday).unwrap();
    content.push_str(render_day("Tuesday", tuesday).as_str());

    let wednesday = meetups_by_day.get(&Day::Wednesday).unwrap();
    content.push_str(render_day("Wednesday", wednesday).as_str());

    let thursday = meetups_by_day.get(&Day::Thursday).unwrap();
    content.push_str(render_day("Thursday", thursday).as_str());

    let friday = meetups_by_day.get(&Day::Friday).unwrap();
    content.push_str(render_day("Friday", friday).as_str());

    let saturday = meetups_by_day.get(&Day::Saturday).unwrap();
    content.push_str(render_day("Saturday", saturday).as_str());

    let sunday = meetups_by_day.get(&Day::Sunday).unwrap();
    content.push_str(render_day("Sunday", sunday).as_str());

    content
}

fn create_nav(focus: &str) -> String {
    let links = vec!["Home", "Groups", "Events and Races", "Routes", "About"]
        .iter()
        .map(|page| {
            let page = page.to_string();
            let href = format!("/{}", match page.as_str() {
                "Home" => "".to_string(),
                "Events and Races" => "events".to_string(),
                page => page.to_lowercase(),
            });

            if page == focus {
                format!(r#"<span class="mr-4 underline underline-offset-4">{page}</span>"#)
            } else {
                format!(r#"<a class="underline decoration-green-400 hover:text-green-400 underline-offset-4 mr-4 leading-8" href="{href}">{page}</a>"#)
            }
        }).collect::<Vec<String>>().join("\n");

    format!(
        r#"<header class="container max-w-3xl mx-auto px-4 pt-16 sm:flex justify-between">
        <h1 class="font-bold text-4xl mb-8 align-bottom">
            <a href="/">Austin Running</a>
        </h1>
        <nav>
            {links}
        </nav>
    </header>"#
    )
}

fn create_html(head: String, navigation: String, content: String) -> String {
    format!(
        r#"<!DOCTYPE html><html>
    {}
    <body>
        {}
        <main class="container max-w-3xl mx-auto px-4 pt-8">
            {}
        </main>
    </body>
</html>"#,
        head, navigation, content
    )
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().ok();

    let spreadsheet_id = dotenv::var("SPREADSHEET_ID").expect("SPREADSHEET_ID must be set");
    let service_account_path =
        dotenv::var("SERVICE_ACCOUNT_PATH").expect("SERVICE_ACCOUNT_PATH must be set");

    let hub = sheet::create_sheets(service_account_path).await;

    let meetups = sheet::get_meetups(&hub, &spreadsheet_id).await.unwrap();
    let index_page = create_html(
        create_head("Austin Running"),
        create_nav("Home"),
        create_meetup_content(&meetups),
    );

    let groups = sheet::get_groups(&hub, &spreadsheet_id).await.unwrap();
    let groups_page = create_html(
        create_head("Austin Running - Groups"),
        create_nav("Groups"),
        create_group_content(&groups),
    );

    let mut file = File::create("./build/index.html").unwrap();
    file.write_all(index_page.as_bytes()).unwrap();
    let mut file = File::create("./build/groups.html").unwrap();
    file.write_all(groups_page.as_bytes()).unwrap();

}
