mod memoria;
mod cpu;
use memoria::Bus;
use cpu::CPU;

fn main() {
    let bus = Bus::new();
    println!("Hello, world!");
}
