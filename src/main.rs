use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name the output name of the csv
    #[arg(short, long)]
    output: String,
    #[arg(short, long, default_value_t = 1)]
    /// give the interface name to scan
    network: u8,
    /// Give the scan time 
    #[arg(short, long, default_value_t = 0)]
    time: u32,
}

fn main() {
    // ASCII art banner
    let banner = r#"
     _________                           
    /   _____/ ____   ____ _____ _______ 
    \_____  \ /  _ \ /    \\__  \\_  __ \
    /        (  <_> )   |  \/ __ \|  | \/
   /_______  /\____/|___|  (____  /__|   
           \/            \/     \/          
    "#;

    println!("{}", banner);

    let args = Args::parse();
    println!("{} {}",args.network, args.output)
}

// Import necessary modules for testing
#[cfg(test)]
mod tests {
    use super::*; // Import items from the main module

    // Test case for parsing command-line arguments
    #[test]
    fn test_parse_args() {
        // Simulate command-line arguments for testing
        let args = Args::parse_from(&["myapp", "-n", "42", "--output", "output.csv"]);

        // Check if the arguments were parsed correctly
        assert_eq!(args.network, 42);
        assert_eq!(args.output, "output.csv");
        assert_eq!(args.time, 0); // Default value
    }
}
