# EN
## Context 
This year, I joined the developpment oriented Application at HELMo
But, I understood that I had some fowarding skills on the basic programmation lessons.
In the mind to train myself on the modern's technics & programs I chose to carry **ALL** my java's projects in rust.

Some links : <br>
- [Theorical](https://github.com/R3D-Corp/HELMo_B1/)
- [Labos](https://github.com/R3D-Corp/HELMo-B1-Labo)

## Technical choices

**Here is the state of the differentes utility classes :**

| Java (HELMo) | Rust (Portage) | État / Crate |
| :--- | :--- | :--- |
| `io.Console` | `tools::console` | Functional + Optimised |
| `util.logs.*` | `logger::*` | Functional but limited |
| `util.Chronometre` | `std::time::Instant` | Unnecessary for the moment |
| `util.Random` | `rand` | [rand](https://docs.rs/rand/) |
| `util.Date` |  `chrono` | [chrono](https://docs.rs/chrono/) |
| `JFrame` | `Slint` | [slint](https://slint.rs) | 


## Structure 
The project is organised to seperate the global logic of the Slint's application and the individuals exercices.

* **`src/main.rs`** : Entrypoint of the launcher (Launcher).
* **`src/style.slint`** : GUI Design.
* **`src/lib.rs`** : Shared modules (Logger, Console, Tools).
* **`src/bin/`** : Executable folder (Exercices & Labos).
    * `cX_*.rs` : theorical exercice X.
    * `lX_*.rs` : partical exercice X.
    * `cX_complex/main.rs` : Exercices which needs multipile files.

## Objectives
The objective after carrying alls programs in rust is to create an Slint application with an integreated console and a fast launch of each exercices after what I will publish an release version.

# FR 

## Contexte
Cette année j'ai rejoint la formation de développement d'application a HELMo.
Cependant, j'ai vite compris que j'avais une avance notable sur les cours de programmation basique.
dans l'esprit de me former au programmes & techniques modernes j'ai choisit de porter **TOUT** mes projets java en rust.

Différents liens : <br>
- [Théorie](https://github.com/R3D-Corp/HELMo_B1/)
- [Labos](https://github.com/R3D-Corp/Helmo-B1-Labo)

## Choix technique

**Voici le statut des classes utilitaires actuellement :**

| Java (HELMo) | Rust (Portage) | État / Crate |
| :--- | :--- | :--- |
| `io.Console` | `tools::console` | Fonctionnel + Optimisé |
| `util.logs.*` | `logger::*` | Fonctionnel mais limité |
| `util.Chronometre` | `std::time::Instant` | Non nécessaire actuellement |
| `util.Random` | `rand` | [rand](https://docs.rs/rand/) |
| `util.Date` |  `chrono` | [chrono](https://docs.rs/chrono/) |
| `JFrame` | `Slint` | [slint](https://slint.rs) | 


## Structure 
Le projet est organisé pour séparer la logique globale de l'application Slint et les exercices individuels.

* **`src/main.rs`** : Point d'entrée de l'application Slint (Launcher).
* **`src/style.slint`** : Design de l'interface graphique.
* **`src/lib.rs`** : Contient les modules partagés (Logger, Console, Tools).
* **`src/bin/`** : Répertoire des exécutables (Exercices & Labos).
    * `cX_*.rs` : Exercices théoriques du Chapitre X.
    * `lX_*.rs` : Travaux pratiques du Labo X.
    * `cX_complex/main.rs` : Exercices nécessitant une structure multi-fichiers.

## Objectifs
L'objectif à la suite d'avoir porter tout les programmes en rust est de : créer une application Slint avec une console intégrée et un démarrage rapide de chaque programme afin d'avoir une version potentiellement 'releaseable'