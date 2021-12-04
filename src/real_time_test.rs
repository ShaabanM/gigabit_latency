use std::fs::{self};

use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

// pub fn sched_loop(freq: f32) {
//     println!("Hello, world!");
// }

/// Runs a loop for a fixed number of iterations using a timer to control bandwidth
///
/// # Arguments
///
/// * `freq` - Required loop frequency
/// * `time` - The duration of the test in seconds
pub fn timer_loop(freq: f32, time: f32) {
    // get the period in seconds
    let period: u64 = (1e+9 / freq) as u64; //nano seconds

    // calculate the number of iterations at a given frequency that span the requested time
    let iterations: usize = (time * freq) as usize;

    // Vector to record all loop start times
    let mut start_times = Vec::new();

    for _i in 0..iterations {
        // Start a timer at the beggining of the loop
        let start = SystemTime::now();

        // record start time
        start_times.push(
            start
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards!")
                .as_nanos() as u64,
        );

        // Time elapsed since loop began
        let elapsed = start.elapsed().expect("time went backwards!").as_nanos() as u64;

        // sleep if the loop executed faster than requested cadance
        if elapsed < period {
            let dur = Duration::from_nanos(period - elapsed);
            thread::sleep(dur)
        }
    }

    // write the start times vector to a file by converting to string first
    let write_strings: Vec<String> = start_times.iter().map(|n| n.to_string()).collect();
    fs::write("start_times", write_strings.join(", ")).expect("Unable to write file");
}
