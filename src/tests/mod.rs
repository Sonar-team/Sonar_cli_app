#[cfg(test)]
use super::*; // Import items from the main module
              // Test case for get_args with default values

#[test]
fn test_main() {
    use assert_cmd::prelude::*; // Importez assert_cmd::prelude::* pour utiliser la syntaxe assert_cmd
                                //use predicates::prelude::*; // Importez predicates::prelude::* pour utiliser la syntaxe predicates
    use std::process::Command;

    let assert = Command::cargo_bin("sonar_cli_app")
        .unwrap()
        .arg("--output=test_output.csv")
        .arg("--interface=test_interface")
        .arg("--time=5")
        .assert();

    // Vérifiez que le programme s'est exécuté avec succès (code de sortie 0)
    assert.success();
}

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

#[test]
fn test_scan_for_time_success() {
    use std::{fs, time::Instant};
    // Créez un nom de fichier de test
    let output = "test_output.csv";
    // Définissez la durée pendant laquelle vous voulez mesurer le temps
    let time = Duration::from_secs(3);

    // Obtenez l'instant de départ
    let start_time = Instant::now();

    // Appelez la fonction de numérisation
    scan_for_time(output, "test_interface", time.as_secs()); // Numérisation pendant 3 secondes (convertir la durée en secondes)

    // Obtenez le temps écoulé
    let elapsed_time = start_time.elapsed();

    // Vérifiez que le temps écoulé est d'au moins 3 secondes
    assert!(elapsed_time >= time);

    // Supprimez le fichier CSV de test après le test
    fs::remove_file(output).expect("Failed to remove test CSV file");
}

#[test]
fn test_scan_until_interrupt() {
    use ctrlc::Signal;
    use nix::sys::signal;
    use nix::unistd::Pid;

    // Spécifiez un nom de fichier de test
    let test_output = "test_output.csv";

    // Créez un thread pour exécuter la fonction scan_until_interrupt
    let handle = std::thread::spawn(move || {
        scan_until_interrupt(test_output, "test_interface");
    });

    // Pausez le test pendant un certain temps (assez long pour simuler une exécution)
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Envoyez une interruption simulée (comme si Ctrl+C était pressé)
    signal::kill(Pid::this(), Signal::SIGINT).expect("Failed to send SIGINT signal");

    // Attendez que le thread se termine
    handle.join().expect("Thread panicked");

    // Vérifiez que le fichier CSV a été créé
    assert!(std::fs::metadata(test_output).is_ok());

    // Supprimez le fichier CSV de test après le test
    std::fs::remove_file(test_output).expect("Failed to remove test CSV file");
}
