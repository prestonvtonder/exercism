const BASE_CARS_PRODUCED: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = success_rate(speed);
    let speed = speed as f64;
    speed * BASE_CARS_PRODUCED * rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}

fn success_rate(speed: u8) -> f64 {
    match speed {
        0       => 0.0,
        1..=4   => 1.0,
        5..=8   => 0.9,
        9..=10  => 0.77,
        11_u8..=u8::MAX => panic!("Invalid speed!")
    }
}
