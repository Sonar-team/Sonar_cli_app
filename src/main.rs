use std::{
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
    thread::{self, sleep},
    time::Duration, process,
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
        println!("Scanning for {} seconds...", time);
        let duration = Duration::from_secs(*time);
        sleep(duration);
    } else {
        println!("Press Ctrl+C to exit...");
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            println!("Ctrl+C pressed. Exiting...");
            r.store(false, SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        while running.load(SeqCst) {
            // Continue running until Ctrl+C is pressed
            thread::sleep(Duration::from_secs(1));
        }
    }

    if let Err(err) = create_csv(output) {
        println!("{}", err);
        process::exit(1);
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

        let (output, interface, time) = get_args(&args);

        assert_eq!(output, "custom_output");
        assert_eq!(interface, "any");
        assert_eq!(time, &10);
    }
    // Test case for print_banner
    #[test]
    fn test_print_banner() {
        let banner = print_banner();

        assert_eq!(banner, banner);
    }

    #[test]
    fn test_create_csv() {
        // Spécifiez un chemin de fichier pour le test
        let test_output = "test_output.csv";

        // Appelez la fonction que vous voulez tester
        let result = create_csv(test_output);

        // Vérifiez que le résultat est Ok, ce qui signifie que la création du fichier CSV a réussi
        assert!(result.is_ok());

        // Vous pouvez également vérifier que le fichier CSV a été créé en vérifiant son existence ou son contenu.
        // Par exemple, vous pouvez utiliser std::fs::metadata pour vérifier l'existence du fichier.

        // Supprimez le fichier CSV de test après le test
        std::fs::remove_file(test_output).expect("Failed to remove test CSV file"); 
    }
}
