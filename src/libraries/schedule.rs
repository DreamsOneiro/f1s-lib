use crate::{Races, time};

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

