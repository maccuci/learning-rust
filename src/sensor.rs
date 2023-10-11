use std::{time::{Duration, Instant}, thread};

use rand::Rng;

pub fn run() {
    let sensor_count = 5;
    let interval = Duration::from_secs(1);
    let simulation = Duration::from_secs(10);

    println!("Iniciando simulação de sensores...");

    let start_time = Instant::now();
    while start_time.elapsed() < simulation {
        let timestamp = Instant::now();

        for sensor_id in 0..sensor_count {
            let value = simulate_data(sensor_id);
            println!("Sensor {} - Valor {} - Tempo: {:?}", sensor_id, value, timestamp.elapsed());
        }

        thread::sleep(interval);
    }
    println!("Simulação finalizada.");
}

fn simulate_data(_id: u32) -> f64 {
    let min = 0.0;
    let max = 100.0;
    let mut rng = rand::thread_rng();
    let value = rng.gen_range(min..=max);
    value
}