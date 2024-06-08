use std::{fs::DirEntry, time::SystemTime};

use chrono::{DateTime, Datelike, Local};

use super::{Matcher, Traverse};

#[derive(Debug)]
pub struct FileDate(pub u16, pub u16, pub u16);
pub struct FileDateMatcher {
    date_range: (FileDate, FileDate),
}

impl FileDateMatcher {
    pub fn new(date_range: (FileDate, FileDate)) -> Self {
        Self { date_range }
    }

    fn date_in_range(&self, date: SystemTime) -> bool {
        let date_min = [
            self.date_range.0 .0 as u32, // Year
            self.date_range.0 .1 as u32, // Month
            self.date_range.0 .2 as u32, // Day
        ];
        let date_max = [
            self.date_range.1 .0 as u32, // Year
            self.date_range.1 .1 as u32, // Month
            self.date_range.1 .2 as u32, // Day
        ];

        let date: DateTime<Local> = DateTime::from(date);

        let date = [date.year() as u32, date.month(), date.day()];

        for i in 0..3 {
            if date_min[i] <= date[i] && date[i] <= date_max[i] {
                continue;
            } else {
                return false;
            }
        }

        true
    }
}

impl Matcher for FileDateMatcher {
    fn process(&self, entry: &DirEntry) -> Traverse {
        let path = entry.path();

        if path.is_dir() {
            Traverse::Recurse
        } else {
            if let Ok(metadata) = path.metadata() {
                let date = metadata.created().unwrap();
                if self.date_in_range(date) {
                    println!("{:?}", path.to_str().unwrap());
                }
            }
            Traverse::NoRecurse
        }
    }
}
