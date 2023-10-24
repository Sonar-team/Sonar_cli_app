use std::{
    process,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
    thread::sleep,
    time::Duration,
};

use capture_packet::{all_interfaces, one_interface};
use clap::Parser;
use csv::Writer;

mod capture_packet;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Name the output name of the csv
    #[arg(short, long, default_value = "output.csv")]
    output: String,
    #[arg(short, long, default_value = "all")]
    /// give the interface name to scan
    interface: String,
    /// Give the scan time
    #[arg(short, long, default_value_t = 0)]
    time: u64,
}

pub fn get_args(args: &Args) -> (&String, &String, &u64) {
    (&args.output, &args.interface, &args.time)
}

pub fn print_banner() -> String {
    // ASCII art banner
    let banner = r"
    _________                           
   /   _____/ ____   ____ _____ _______ 
   \_____  \ /  _ \ /    \\__  \\_  __ \
   /        (  <_> )   |  \/ __ \|  | \/
  /_______  /\____/|___|  (____  /__|   
          \/            \/     \/          
   ";

    banner.to_string()
}

pub fn create_csv(output: &str) -> Result<(), Box<dyn std::error::Error>> {
    // creat a csv file
    let mut writer = Writer::from_path(output)?;
    // Fermez le fichier CSV (c'est important pour garantir que les données sont écrites)
    writer.flush()?;
    Ok(())
}

pub fn scan_for_time(output: &str, interface: &str, time: u64) {
    println!("Scanning {} for {} seconds...", interface, time);

    let duration = Duration::from_secs(time);
    sleep(duration);
    interfaces_handler(interface);

    match create_csv(output) {
        Ok(_) => {
            println!("Scan completed successfully. CSV file created.");
        }
        Err(err) => {
            eprintln!("Error creating CSV file: {}", err);
            process::exit(1);
        }
    }
}

pub fn scan_until_interrupt(output: &str, interface: &str) {
    println!("Scanning {} ...", interface);
    println!("Press Ctrl+C to exit...");
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let output_clone = output.to_string();

    ctrlc::set_handler(move || handle_interrupt(r.clone(), &output_clone))
        .expect("Error setting Ctrl-C handler");

    while running.load(SeqCst) {
        // Continue running until Ctrl+C is pressed
        interfaces_handler(interface);
    }
}

// This new function encapsulates what should happen when Ctrl+C is pressed.
pub fn handle_interrupt(r: Arc<AtomicBool>, output: &str) {
    println!("Ctrl+C pressed. Exiting...");
    r.store(false, SeqCst);
    match create_csv(output) {
        Ok(_) => {
            println!("Scan completed successfully. CSV file created.");
        }
        Err(err) => {
            eprintln!("Error creating CSV file: {}", err);
            process::exit(1);
        }
    }
}

fn interfaces_handler(interface: &str) {
    match check_interface(interface) {
        true => all_interfaces(),
        false => one_interface(interface),
    }
}

fn check_interface(interface: &str) -> bool {
    match interface {
        "all" => true,
        _ => false,
    }
}



mod tests_unitaires;
