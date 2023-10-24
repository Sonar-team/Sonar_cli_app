[![License Badge]](./LICENSE)
[![CodecovBadge]](https://app.codecov.io/gh/Sonar-team/Sonar_cli_app)
# Sonar - Surveillance Optimisée des Nœuds pour Analyse Réseau Sécurisé

## Description

Sonar est une application CLI conçue pour la surveillance optimisée et la sécurisation des réseaux. Elle capture des paquets réseau et stocke des informations spécifiques à leur sujet dans un format de données structuré. Utilisant la bibliothèque `pnet` pour la capture de paquets, Sonar extrait des détails tels que le type Ether, les adresses IP sources et de destination, et le protocole utilisé. Chaque paquet unique est ensuite stocké dans une `HashSet` pour éviter les doublons.

## Caractéristiques

- Capture de paquets réseau grâce à `pnet`
- Extraction de données à partir des paquets capturés
- Stockage des informations des paquets dans un DataFrame via `polars` (en cours de développement)
- Évitement des doublons grâce à `HashSet`

## Prérequis

- Rust
- Cargo
- libpnet
- polars (en cours de développement)

## Installation

1. Clonez le dépôt :
   ```bash
   git clone https://github.com/votre-nom-d-utilisateur/sonar.git
   ```
2. Accédez au répertoire du projet :
   ```bash
   cd sonar
   ```
3. Compilez le projet :
   ```bash
   cargo build --release
   ```

## Utilisation

Exécutez l'application avec la commande suivante :

```bash
sudo ./target/release/sonar
```

## Contribution

Les pull requests sont les bienvenues. Pour des changements majeurs, veuillez ouvrir une issue au préalable pour en discuter.

## Licence

[MIT](LICENSE.md)
