#![no_main]
#![no_std]

use marcus_wedding_box as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    marcus_wedding_box::exit()
}
