extern crate sonar_cli_app;

use clap::Parser;
use sonar_cli_app::{
    print_banner,
    get_args,
    scan_for_time,
    scan_until_interrupt,
    Args
};

fn main() {
    println!("{}", print_banner());

    let args = Args::parse();
    let (output, interface, time) = get_args(&args);
    println!(
        "Output: {}, Interface: {}, Time: {}",
        output, interface, time
    );

    if time > &0 {
        scan_for_time(output, interface, *time);
    } else {
        scan_until_interrupt(output, interface);
    }
}

