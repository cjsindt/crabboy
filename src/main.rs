use crabboy::dmgcpu::DMGCPU;
use std::time::Duration;
use std::thread;

const CPU_SPEED: u32 = 4_190_000;   // cpu clock speed in Hz

fn main() {

    let mut gbc = DMGCPU::new(CPU_SPEED);
    gbc.run();

    println!("Total clock cycles: {}", gbc.get_cpu_clock().get_total_cycles());
    println!("Total cpu cycles: {}", gbc.get_cycle_count());
}
