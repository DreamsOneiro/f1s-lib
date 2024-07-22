use crate::{Races, time};

// Search and return index of current race
pub fn search_current(races: &Vec<Races>) -> Option<usize> {
    let time_now = time::utc_now();
    for (i, race) in races.iter().enumerate() {
        if time_now < race.main_race() {
            return Some(i);
        }
    }
    None
}
