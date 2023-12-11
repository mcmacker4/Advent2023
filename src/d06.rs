

pub struct Race {
    pub time: i64,
    pub record: i64,
}

pub fn calculate_race(race: &Race) -> (f64, f64) {
    let min = minimum(race.time, race.record);
    let max = maximum(race.time, race.record);
    (min, max)
}

fn minimum(time: i64, min: i64) -> f64 {
    let rad = radical(time, min);
    f64::floor((time as f64 - rad) / 2.0)
}

fn maximum(time: i64, min: i64) -> f64 {
    let rad = radical(time, min);
    f64::ceil((time as f64 + rad) / 2.0)
}

fn radical(time: i64, min: i64) -> f64 {
    let inner = time.pow(2) - 4 * min;
    f64::sqrt(inner as f64)
}

