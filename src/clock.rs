use std::thread;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Clock {
    total_cycles: Arc<Mutex<u64>>,
    clock_speed: Arc<u32>,
}

impl Clock {
    pub fn new(speed: u32) -> Clock {
        Clock {
            total_cycles: Arc::new(Mutex::new(0)), // Initialize total_cycles
            clock_speed: Arc::new(speed),
        }
    }

    // start the clock in a separate thread
    pub fn start(&self) {
        let total_cycles = Arc::clone(&self.total_cycles);
        let clock_speed = Arc::clone(&self.clock_speed);

        thread::spawn(move || {
            let period = 1_000_000_000u64 / (*clock_speed as u64); // Convert Hz to nanoseconds
            
            let mut last_time = Instant::now();
            let nanoseconds_per_cycle = Duration::from_nanos(period);

            // busy-wait loop to emulate timing
            loop{
                while Instant::now().duration_since(last_time) < nanoseconds_per_cycle {
                    thread::yield_now();
                }

                let mut cycles = total_cycles.lock().unwrap();
                *cycles += 1;
                last_time += nanoseconds_per_cycle;
            }
        });
    }

    pub fn get_total_cycles(&self) -> u64 {
        let cycles = self.total_cycles.lock().unwrap();
        *cycles
    }
}
