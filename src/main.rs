extern crate sonar_cli_app;

use std::process;

use clap::Parser;
use sonar_cli_app::{get_args, print_banner, scan_for_time, scan_until_interrupt, Args};

fn main() {
    println!("{}", print_banner());

    let args = Args::parse();
    let (output, interface, time) = get_args(&args);
    println!(
        "Output: {}, Interface: {}, Time: {}",
        output, interface, time
    );

    let result = if *time > 0 {
        scan_for_time(output, interface, *time)
    } else {
        scan_until_interrupt(output, interface)
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
