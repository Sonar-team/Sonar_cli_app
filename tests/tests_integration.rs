#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_run_app_with_options() {
    use std::process::Command;
    // Run the application with custom arguments
    let output = Command::new("target/debug/sonar_cli_app")
        .arg("--output=my_output.csv")
        .arg("--interface=any")
        .arg("--time=1")
        .output()
        .expect("Failed to run command");

    assert!(output.status.success()); // Check for successful exit
    // Add more checks on `output.stdout` or `output.stderr` if you want
}

#[test]
fn test_run_app_with_defaults() {
    use std::process::Command;
    // Run the application without any arguments
    let output = Command::new("target/debug/sonar_cli_app")
        .output()
        .expect("Failed to run command");

    assert!(output.status.success()); // Check for successful exit
    // Add more checks on `output.stdout` or `output.stderr` if you want
}


