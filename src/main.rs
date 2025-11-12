use std::{fs, os::unix::process::CommandExt, process::Command};

use colored::Colorize;
use sysinfo::System;
use crossterm::event::{read, Event};

fn main() {
    let mut clear_cmd = Command::new("clear").spawn().expect("failed to clear terminal");
    let _ = clear_cmd.wait();
    let manager = match battery::Manager::new() {
        Ok(m) => m,
        Err(e) => {
            println!("failed to create battery manager: {}", e);
            std::process::exit(1);
        }
    };

    let batteries = match manager.batteries() {
        Ok(bat) => bat,
        Err(e) => {
            println!("failed to get list of batteries: {}", e);
            std::process::exit(1);
        }
    };

    println!("{}", "Battery Info:".cyan());
    for (idx, maybe_battery) in batteries.enumerate() {
        let battery = match maybe_battery {
            Ok(bat) => bat,
            Err(e) => {
                println!("failed to get info for battery #{}", e);
                continue;
            }
        };

        println!("{}", format!("  Battery #{}:", idx + 1).yellow());
        println!("  State:                {:?}", battery.state());
        if let Some(to_full) = battery.time_to_full() {
            let time_to_full = to_full;
            let seconds = time_to_full.value;
            let hours = (seconds / 3600.0).floor();
            let minutes = ((seconds % 3600.0) / 60.0).floor();
            println!("  Time to full charge:  {:.0}h {:.0}m", hours, minutes,);
        }
        if let Some(to_empty) = battery.time_to_empty() {
            let time_to_empty = to_empty;
            let seconds = time_to_empty.value;
            let hours = (seconds / 3600.0).floor();
            let minutes = ((seconds % 3600.0) / 60.0).floor();
            println!("  Time to empty:        {:.0}h {:.0}m", hours, minutes,);
        }
        println!("  Health:               {:.0}%", battery.state_of_health().value * 100.0);
        if let Some(cycles) = battery.cycle_count() {
            println!("  Charge cylces:        {:?}", cycles);
        }
        println!("  Percentage charged:   {:.0}%", battery.state_of_charge().value * 100.0);
    }
    
    println!();

    // system info
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("{}", "Hardware Info:".cyan());

    let total_memory_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    println!("  Memory:               {:.2} GB", total_memory_gb);
    println!("  CPU cores:            {}", sys.cpus().len());
    match fs::read_to_string("/sys/class/dmi/id/product_name") {
        Ok(name) => println!("  Product Name:         {}", name.trim_end()),
        Err(e) => println!("{}", format!("WARNING: failed to get product name: {}", e).red())
    }
    match fs::read_to_string("/sys/class/dmi/id/product_serial") {
        Ok(serial) => println!("  Product Serial:       {}", serial.trim_end()),
        Err(e) => println!("{}", format!("WARNING: failed to get product serial: {}", e).red())
    }
    match fs::read_to_string("/sys/class/dmi/id/bios_version") {
        Ok(version) => println!("  BIOS Version:         {}", version.trim_end()),
        Err(e) => println!("{}", format!("WARNING: failed to get bios version: {}", e).red())
    }
    match fs::read_to_string("/sys/class/dmi/id/bios_date") {
        Ok(date) => println!("  BIOS Date:            {}", date.trim_end()),
        Err(e) => println!("{}", format!("WARNING: failed to get bios date: {}", e).red())
    }


    // println!("\n{}", "  Disks:".yellow());
    // for disk in &Disks::new_with_refreshed_list() {
    //     let name = disk.name().to_string_lossy();
    //     let fs = disk.file_system().to_string_lossy();
    //     let space_gb = disk.total_space() as f64 / (1024.0 * 1024.0 * 1024.0);
    //     println!("    {} ({}): {:.1} GB", name, fs, space_gb);
    // }

    println!("\n\n{}", "Press Enter to shutdown or CTRL+C to continue in tty...".green());
    
    loop {
        if let Event::Key(_) = read().unwrap() {
            break;
        }
    }

    let _ = Command::new("poweroff").exec();
}
