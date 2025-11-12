fn main() {
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

    for (idx, maybe_battery) in batteries.enumerate() {
        let battery = match maybe_battery {
            Ok(bat) => bat,
            Err(e) => {
                println!("failed to get info for battery #{}", e);
                continue;
            }
        };

        println!("Battery #{}:", idx + 1);
        println!("State: {:?}", battery.state());
        if let Some(to_full) = battery.time_to_full() {
            let time_to_full = to_full;
            let seconds = time_to_full.value;
            let hours = (seconds / 3600.0).floor();
            let minutes = ((seconds % 3600.0) / 60.0).floor();
            println!("Time to full charge: {:.0}h {:.0}m", hours, minutes,);
        }
        if let Some(to_empty) = battery.time_to_empty() {
            let time_to_empty = to_empty;
            let seconds = time_to_empty.value;
            let hours = (seconds / 3600.0).floor();
            let minutes = ((seconds % 3600.0) / 60.0).floor();
            println!("Time to empty: {:.0}h {:.0}m", hours, minutes,);
        }
        println!("Health: {:.0}%", battery.state_of_health().value * 100.0);
        if let Some(cycles) = battery.cycle_count() {
            println!("Charge cylces: {:?}", cycles);
        }
        println!("Percentage charged: {}%", battery.state_of_charge().value * 100.0);
    }
}
