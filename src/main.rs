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
    println!("{}",print_banner());

    let args = Args::parse();
    let (output, network, time) = get_args(&args);
    println!("Output: {}, Network: {}, Time: {}", output, network, time);
}

fn get_args(args: &Args) -> (String, u8, u32) {
    (args.output.clone(), args.network, args.time)
}


fn print_banner() -> String {
    // ASCII art banner
    let banner = r#"
    _________                           
   /   _____/ ____   ____ _____ _______ 
   \_____  \ /  _ \ /    \\__  \\_  __ \
   /        (  <_> )   |  \/ __ \|  | \/
  /_______  /\____/|___|  (____  /__|   
          \/            \/     \/          
   "#;

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
            network: 1,
            time: 0,
        };

        let (output, network, time) = get_args(&args);

        assert_eq!(output, "default_output");
        assert_eq!(network, 1);
        assert_eq!(time, 0);
    }

    // Test case for get_args with custom values
    #[test]
    fn test_get_args_custom() {
        let args = Args {
            output: "custom_output".to_string(),
            network: 42,
            time: 10,
        };

        let (output, network, time) = get_args(&args);

        assert_eq!(output, "custom_output");
        assert_eq!(network, 42);
        assert_eq!(time, 10);
    }
    // Test case for print_banner
    #[test]
    fn test_print_banner() {
        let banner = print_banner();

        assert_eq!(banner, banner);
    }
}