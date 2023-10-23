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
    let a = 1;
    let b = 2;
    addition(a, b);
    
    print_banner();

    pars_arguments();
}

fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn print_banner() {
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
}

fn pars_arguments() {
    let args = Args::parse();
    println!("{} {}",args.network, args.output)
}

// Import necessary modules for testing
#[cfg(test)]
mod tests {

    use super::*; // Import items from the main module
    // Test addition function
    #[test]
    fn test_addition() {
        assert_eq!(addition(1, 2), 3);
    }

    // Test case for parsing command-line arguments
    #[test]
    fn test_parse_arguments() {
        // Simulate command-line arguments for testing
        let args = Args::parse_from(&["myapp", "-n", "42", "--output", "output.csv"]);

        // Check if the arguments were parsed correctly
        assert_eq!(args.network, 42);
        assert_eq!(args.output, "output.csv");
        assert_eq!(args.time, 0); // Default value
    }

    // Test case for the main function (if needed)
    // #[test]
    // fn test_main() {
    //     // Add test logic for your main function if necessary
    // }
}

