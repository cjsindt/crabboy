mod clock;

use clock::Clock;
use std::time::Duration;
use std::thread;

const CPU_SPEED: u32 = 4_190_000;   // cpu clock speed in Hz

fn main() {
    // setup clock
    let cpu_clock = Clock::new(CPU_SPEED);
    cpu_clock.start();

    thread::sleep(Duration::from_secs(2));

    println!("Total cycles: {}", cpu_clock.get_total_cycles());
}
