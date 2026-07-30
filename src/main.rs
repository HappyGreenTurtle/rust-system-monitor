use sysinfo::{System, Product};
use std::{thread, time::Duration};
use std::io::{self, Write};

mod cpu;
mod ram;
mod ui;

fn main() {
    let mut system = System::new_all();
    let mut cpu = cpu::Cpu::new();
    
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();

    loop {
        // Clear terminal first
        print!("\x1B[H");

        // Static info
        println!("System name:           {:?}", System::name().unwrap());
        println!("Model:                 {:?}", Product::version().unwrap());
        println!("Kernel version:        {:?}", System::kernel_version().unwrap());
        println!("Host name:             {:?}", System::host_name().unwrap());
        println!("# of CPU cores:        {:?}", cpu::Cpu::num_of_cpu(&system));

        println!();

        system.refresh_cpu_all();
        system.refresh_memory();

        let cpu_usage = cpu.update(&system);
        let (ram_used, ram_total) = ram::get_ram(&mut system);

        let ram_percent = (ram_used as f32 / ram_total as f32) * 100.0;

        println!(
            "{:<24} {:.1}% [{}]",
            "CPU:",
            cpu_usage,
            ui::bar(cpu_usage)
        );

        println!(
            "{:} {} MB / {} MB {:.1}% [{}]",
            "RAM:",
            ram_used,
            ram_total,
            ram_percent,
            ui::bar(ram_percent)
        );

        thread::sleep(Duration::from_millis(1000));
    }
}
