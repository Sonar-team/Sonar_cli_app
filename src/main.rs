use std::{
    process,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
    thread::{self, sleep},
    time::Duration,
};

use clap::Parser;
use csv::Writer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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

fn main() {
    println!("{}", print_banner());

    let args = Args::parse();
    let (output, interface, time) = get_args(&args);
    println!("Output: {}, Network: {}, Time: {}", output, interface, time);

    if time > &0 {
        scan_for_time(output, interface, *time);
    } else {
        scan_until_interrupt(output, interface);
    }
}

fn get_args(args: &Args) -> (&String, &String, &u64) {
    (&args.output, &args.interface, &args.time)
}

fn print_banner() -> String {
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

fn create_csv(output: &str) -> Result<(), Box<dyn std::error::Error>> {
    // creat a csv file
    let mut writer = Writer::from_path(output)?;
    // Fermez le fichier CSV (c'est important pour garantir que les données sont écrites)
    writer.flush()?;
    Ok(())
}

fn scan_for_time(output: &str, interface: &str, time: u64) {
    println!("Scanning {} for {} seconds...", interface, time);

    let duration = Duration::from_secs(time);
    sleep(duration);

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

fn scan_until_interrupt(output: &str, interface: &str) {
    println!("Scanning {} ...", interface);
    println!("Press Ctrl+C to exit...");
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let output_clone = output.to_string(); // Clonez output pour qu'il soit 'static'
    ctrlc::set_handler(move || {
        println!("Ctrl+C pressed. Exiting...");
        r.store(false, SeqCst);
        match create_csv(&output_clone) { // Utilisez output_clone ici
            Ok(_) => {
                println!("Scan completed successfully. CSV file created.");
            }
            Err(err) => {
                eprintln!("Error creating CSV file: {}", err);
                process::exit(1);
            }
        }
    })
    .expect("Error setting Ctrl-C handler");

    while running.load(SeqCst) {
        // Continue running until Ctrl+C is pressed
        thread::sleep(Duration::from_secs(1));
    }
}

mod tests;
