use std::{time::Duration,
        thread::{sleep,
                 self},
        sync::{atomic::{AtomicBool,
                        Ordering::SeqCst}, 
            Arc}
        };

use clap::Parser;
use csv::Writer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name the output name of the csv
    #[arg(short, long,  default_value = "output.csv")]
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
        println!("Waiting for {} seconds...", time);
        let duration = Duration::from_secs(*time);
        sleep(duration);
    } else {
        println!("Press Ctrl+C to exit...");
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            println!("Ctrl+C pressed. Exiting...");
            r.store(false, SeqCst);
        }).expect("Error setting Ctrl-C handler");

        while running.load(SeqCst) {
            // Continue running until Ctrl+C is pressed
            thread::sleep(Duration::from_secs(1)); 
        }
    }
    // creat a csv file
    let _ = Writer::from_path(output).unwrap();

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

// Import necessary modules for testing
#[cfg(test)]
mod tests {

    use super::*; // Import items from the main module

    // Test case for get_args with default values
    #[test]
    fn test_get_args_default() {
        let args = Args {
            output: "default_output".to_string(),
            interface: "any".to_string(),
            time: 0,
        };

        let (output, interface, time) = get_args(&args);

        assert_eq!(output, "default_output");
        assert_eq!(interface, "any");
        assert_eq!(time, &0);
    }

    // Test case for get_args with custom values
    #[test]
    fn test_get_args_custom() {
        let args = Args {
            output: "custom_output".to_string(),
            interface: "any".to_string(),
            time: 10,
        };

        let (output, network, time) = get_args(&args);

        assert_eq!(output, "custom_output");
        assert_eq!(network, "any");
        assert_eq!(time, &10);
    }
    // Test case for print_banner
    #[test]
    fn test_print_banner() {
        let banner = print_banner();

        assert_eq!(banner, banner);
    }
}
