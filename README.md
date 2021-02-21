# Aprendo

## Table of Contents
1. [Purpose](https://github.com/Malique-Auguste/Aprendo#purpose)
2. [Stucture Overview](https://github.com/Malique-Auguste/Aprendo#structure-overview)
    * [Environmnet](https://github.com/Malique-Auguste/Aprendo#environment)
    * [Dictionary](https://github.com/Malique-Auguste/Aprendo#dictionary)
    * [Phrase](https://github.com/Malique-Auguste/Aprendo#phrase)
3. [Getting Started](https://github.com/Malique-Auguste/Aprendo#getting-started)
4. [Dependencies](https://github.com/Malique-Auguste/Aprendo#dependencies)

## Purpose
Aprendo seeks to function as an easily extensiable command line tool to help futher my studies in Spanish (it will not be difficult to allow the translating of other languages as long as they are supported by the _my memory api_). Currently it uses the my memory api to translate any phrase.

## Structure Overview
The program is split up into three main structs (classes):
1. [Environment](src/environment.rs)
2. [Dictionary](src/dictionary.rs)
3. [Phrase](src/phrase.rs)

### Environment
This is the main class that the user interacts with. It handles all user input and is responsible for testing the user using the self-contined Dictioanry. It manages the state of the program.

### Dictionary
The dicitnary is a simple wrapper over a _HashMap_ that is used to store phrases.

### Phrase
The word struct functionas similarly to a flash-card but also stores data such as when last the card was tested and its difficulty. This extra data is used by the Environment in the testing of the user's vocabulary.

_Note: Helper structs are present such as [Language](src/phrase.rs) and [Translation](src/translation.rs). They mainly function to help other structs perform their duties and are for the most part useless on their own._

## Getting Started
**This repository is not currently in a state where i encourage it's use but of course I can't stop you ;)**

## Dependencies
* reqwest = "0.11"
* tokio = "1"
* serde = "1.0.123"
* serde_json = "1.0"