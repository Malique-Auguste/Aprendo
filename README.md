# Aprendo

## Purpose
Aprendo seeks to function as an easily extensiable command line tool to help futher my studies in Spanish (it will not be difficult to allow the translating of other languages as long as they are supported by the _my memory api_). Currently it uses the my memory api to translate any phrase.

---

## Structure Overview
The program is split up into three main structs (classes):
1) Environment
2) Dictionary
3) Word

### Environment
This is the main class that the user interacts with. It handles all user input and is responsible for testing the user from the self contined Dictioanry.

### Dictionary
The dicitnary is a simple wrapper over a _HashMap_.

### Word
The word struct functionas similarly to a flash-card but also stores data such as when last the card was tested and its difficulty. This extra data is used by the environment in the testing of the user's vocabulary.

_Note: Helper structs are present such as *Language* and *Translation*. They mainly function to assist other structs perform their duties and are for the most part useless on their own._

---