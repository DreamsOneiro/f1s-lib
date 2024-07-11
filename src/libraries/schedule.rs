use crate::{Races, time, Race};

// Search and return index of current race
pub fn search_current(races: &Vec<Races>) -> Option<usize> {
    let time_now = time::utc_now();
    for (i, race) in races.iter().enumerate() {
        let race_time = time::to_utc(&race.date, &race.time);
        if time_now < race_time {
            return Some(i);
        }
    }
    None
}

// Search and return index of current race
pub fn find_current(races: &Vec<Race>) -> Option<usize> {
    let time_now = time::utc_now();
    for (i, race) in races.iter().enumerate() {
        if time_now < race.main_race() {
            return Some(i);
        }
    }
    None
}
