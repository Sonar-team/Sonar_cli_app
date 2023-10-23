#[cfg(test)]
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