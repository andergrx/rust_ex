use std::fmt;

use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, TimeDelta};
use reqwest::Client;
use serde_json::Value;

#[derive(Debug)]
pub struct Personal {
    days: i64,
    days_to_next_bday: i64,
    du_bday_wife: i64,
    du_bday_kai: i64,
    du_bday_kara: i64,
    du_halloween: i64,
    du_thanksgiving: i64,
    day_thanksgiving: u16,
    du_xmas: i64,
    daily_message: String,
}

impl Personal {
    pub fn new() -> Self {
        let start = NaiveDate::from_ymd_opt(2024, 9, 11)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();

        let today = Local::now();

        let new_year = NaiveDate::from_ymd_opt(today.naive_local().year() + 1, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();

        let span = new_year - today;
        let mut year = today.naive_local().year();
        if span.num_days() < 365 {
            year += 1;
        }

        let bday = NaiveDate::from_ymd_opt(year, 6, 18)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();
        let bday_wife = NaiveDate::from_ymd_opt(year, 9, 9)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();
        let bday_kai = NaiveDate::from_ymd_opt(year, 12, 26)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();
        let bday_kara = NaiveDate::from_ymd_opt(year, 7, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();
        let halloween = NaiveDate::from_ymd_opt(year, 10, 31)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();
        let xmas = NaiveDate::from_ymd_opt(year, 12, 25)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();
        let (thanksgiving, nov_day) = Self::find_thanksgiving(&today, year);

        let duration = today - start;
        let days_until = Self::days_until(bday - today);
        let days_until_wife = Self::days_until(bday_wife - today);
        let days_until_kai = Self::days_until(bday_kai - today);
        let days_until_kara = Self::days_until(bday_kara - today);
        let days_until_halloween = Self::days_until(halloween - today);
        let days_until_thanksgiving = Self::days_until(thanksgiving - today);
        let days_until_xmas = Self::days_until(xmas - today);

        Self {
            days: duration.num_days(),
            days_to_next_bday: days_until.num_days() + 1,
            du_bday_wife: days_until_wife.num_days() + 1,
            du_bday_kai: days_until_kai.num_days() + 1,
            du_bday_kara: days_until_kara.num_days() + 1,
            du_halloween: days_until_halloween.num_days() + 1,
            du_thanksgiving: days_until_thanksgiving.num_days() + 1,
            day_thanksgiving: nov_day as u16,
            du_xmas: days_until_xmas.num_days() + 1,
            daily_message: String::from(""),
        }
    }

    pub async fn generate_message(&mut self) {
        let client = Client::new();
        let response = client
            .get("https://www.affirmations.dev/")
            .send()
            .await
            .expect("Request failed.");

        let text = response.text().await.unwrap();
        let json: Value = serde_json::from_str(&text).expect("Json convert failed");

        self.daily_message = json["affirmation"].to_string();
    }

    fn days_until(span: TimeDelta) -> TimeDelta {
        let mut days = span;
        let year = Duration::new(60 * 60 * 24 * 365, 0).unwrap();
        if days > year {
            days -= year;
        }
        days
    }

    fn find_thanksgiving(today: &DateTime<Local>, year_in: i32) -> (DateTime<Local>, u16) {
        let mut year = year_in;
        let mut nov_1 = NaiveDate::from_ymd_opt(year, 11, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Local)
            .unwrap();

        if (*today - nov_1).num_days() > 365 {
            year -= 1;
            nov_1 = NaiveDate::from_ymd_opt(year, 11, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Local)
                .unwrap();
        }
        let mut day_i: i32 = nov_1.weekday().num_days_from_monday() as i32 - 3;
        let mut nov_day: u16 = 1;
        if day_i > 0 {
            day_i -= 7;
        }
        while day_i != 0 {
            nov_day += 1;
            day_i += 1;
        }
        nov_day += 21;
        if nov_day >= 29 {
            nov_day -= 7;
        }

        (
            NaiveDate::from_ymd_opt(year, 11, nov_day as u32)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Local)
                .unwrap(),
            nov_day,
        )
    }

    pub fn days(&self) -> i64 {
        self.days
    }
}

impl fmt::Display for Personal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Days: {}\n\
            Days until Birthday: {}\n\
            Days until Wife's Bday: {}\n\
            Days until Kai's Bday: {}\n\
            Days until Kara's Bday: {}\n\
            Days until Halloween: {}\n\
            Days until Thanksgiving: {} (11/{})\n\
            Days until Christmas: {}\n\
            Message: {}",
            self.days,
            self.days_to_next_bday,
            self.du_bday_wife,
            self.du_bday_kai,
            self.du_bday_kara,
            self.du_halloween,
            self.du_thanksgiving,
            self.day_thanksgiving,
            self.du_xmas,
            self.daily_message
        )
    }
}
