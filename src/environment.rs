use crate::dictionary::Dictionary;
use crate::phrase::{Language, Phrase};
use crate::translation::translate;
use crate::helper::{get_rand_unique_indices, shuffle};

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    dict: Dictionary,
}

impl Environment {
    pub fn new(path: Option<&str>) -> Result<Environment, (Environment, String)> {
        match Dictionary::new(path) {
            Ok(dict) => Ok(Environment { dict }),
            Err(e) => Err((Environment { dict: e.0 }, e.1))
        }
    }

    pub fn start(&mut self) {
        println!(
            "Welcome to Aperndo!
            \rThis is a command line interace made to handle all your translation needs."
        );

        loop {
            println!(
                "\nSelect an action.
                \r  1 - Add a single phrase to the dictionary.
                \r  2 - Add multiple phrases to the dictionary.
                \r  3 - Translate a phrase.
                \r  4 - Test a random curation of phrases.
                \r  5 - Test a specific topic.
                \r  6 - Test most difficult phrases.
                \r  7 - Supported Languages and ISO 639-1 Codes
                \r  8 - HELP!
                \r  9 - Exit"
            );

            let mut option = String::new();
            if let Err(e) = Environment::read_input(&mut option) {
                println!("Err: {}\n Exiting program.", e);
                break;
            };
            let result = match option.parse() {
                Ok(i) => match i {
                    1 => self.add_phrase(),
                    2 => self.add_multiple_phrases(),
                    3 => self.translate(),
                    4 => self.test_random(),
                    5 => Err("Not yet implemented".into()),
                    6 => Err("Not yet implemented".into()),
                    7 => Err("Not yet implemented".into()),
                    8 => Err("Not yet implemented".into()),
                    9 => break,
                    _ => Err("Not a valid option.".into()),
                },
                Err(_) => Err(format!(
                    "'{}' is not valid input(select a number to begin action).",
                    option
                )),
            };

            if let Err(e) = result {
                println!("\nErr: {}", e);
            }
        }
    }

    pub fn add_multiple_phrases(&mut self) -> Result<(), String> {
        let mut topic = String::new();
        println!("\nEnter the topic that the phrases fall under (e.g foods, sports, travel). If n/a type press enter (type nothing): ");
        Environment::read_input(&mut topic)?;

        println!("\nEnter the phrases' ISO 639-1 language code: ");
        let input_lang = Environment::read_language()?;

        println!("\nEnter the target ISO 639-1 language code: ");
        let output_lang = Environment::read_language()?;

        loop {
            let mut phrase = String::new();
            println!("\nEnter phrase to be translated: ");
            Environment::read_input(&mut phrase)?;

            if phrase == String::new() {
                break;
            }

            let translation = match translate(&phrase, &input_lang, output_lang, true) {
                Ok(t) => t,
                Err(e) => return Err(format!("Unable to translate because: {}", e)),
            };

            let phrase = Phrase::new(phrase, input_lang, topic.clone(), vec![translation]);

            self.dict.insert(phrase.spelling.clone(), phrase);
        }

        Ok(())
    }

    pub fn add_phrase(&mut self) -> Result<(), String> {
        let mut phrase = String::new();
        println!("\nEnter the phrase to be translated: ");
        Environment::read_input(&mut phrase)?;

        let mut topic = String::new();
        println!("\nEnter the topic that the phrase falls under (e.g foods, sports, travel). If n/a type press enter (type nothing): ");
        Environment::read_input(&mut topic)?;

        println!("\nEnter the phrase's ISO 639-1 language code: ");
        let input_lang = Environment::read_language()?;

        println!("\nEnter the target ISO 639-1 language code: ");
        let output_lang = Environment::read_language()?;

        let translation = match translate(&phrase, &input_lang, output_lang, true) {
            Ok(t) => t,
            Err(e) => return Err(format!("Unable to translate because: {}", e)),
        };

        let phrase = Phrase::new(phrase, input_lang, topic, vec![translation]);

        self.dict.insert(phrase.spelling.clone(), phrase);

        Ok(())
    }

    pub fn test_random(&mut self) -> Result<(), String> {
        let mut lang_group = String::new();
        println!("Enter a group of ISO 639-1 language codes(en|es): ");
        Environment::read_input(&mut lang_group)?;
        let lang_group: Vec<&str> = lang_group.split('|').collect();

        let lang_group = {
            let mut temp_lang_group: Vec<Language> = Vec::new();
            for lang in lang_group.iter() {
                temp_lang_group.push(match Language::from_str(lang) {
                    Ok(l) => l,
                    Err(_) => return Err("Invalid language code entered.".into()),
                });
            }

            temp_lang_group
        };

        let lang_group_size = self.dict.get_lang_group(&lang_group).len();
        if lang_group_size < 5 {
            return Err("There are less than 5 phrases with that group of language codes.".into());
        }

        let mut test_length = String::new();
        println!("\nYour dictionary currently holds {} phrases with those language codes. Enter how many of them you would like to test: ", lang_group_size);
        Environment::read_input(&mut test_length)?;
        let test_length = match test_length.parse() {
            Ok(n) => n,
            Err(e) => return Err(format!("{:?}", e)),
        };

        if test_length < 5 {
            return Err("You must choose to test more than 4 phrases.".into());
        }

        let test_phrases = self.dict.get_test_phrases(test_length, &lang_group);

        println!("Select a, b, c or d to choose the translation.");
        for i in 0..test_phrases.len() {           
            let mut indices = get_rand_unique_indices(test_phrases.len(), 4, Some(i))?;

            println!("{}", test_phrases[i]);
            println!("a) {}", test_phrases[indices[0]]);
            println!("b) {}", test_phrases[indices[1]]);
            println!("c) {}", test_phrases[indices[2]]);         
            println!("d) {}\n", test_phrases[indices[3]]); 
        }

        Ok(())
    }

    fn read_input(s: &mut String) -> Result<(), String> {
        if let Err(e) = io::stdin().read_line(s) {
            return Err(format!("Unable to read lang due to: {}", e));
        }

        //trims new line
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }

        Ok(())
    }

    fn read_language() -> Result<Language, String> {
        let mut lang = String::new();
        Environment::read_input(&mut lang)?;

        match Language::from_str(&lang) {
            Ok(l) => Ok(l),
            Err(_) => Err(format!(
                "{} is not a valid code translatable by this program.",
                lang
            )),
        }
    }

    pub fn translate(&self) -> Result<(), String> {
        let mut phrase = String::new();
        println!("\nEnter the phrase to be translated: ");
        Environment::read_input(&mut phrase)?;

        println!("\nEnter the phrase's ISO 639-1 language code: ");
        let input_lang = Environment::read_language()?;

        println!("\nEnter the target ISO 639-1 language code: ");
        let output_lang = Environment::read_language()?;

        let translation = match translate(&phrase, &input_lang, output_lang, true) {
            Ok(t) => t,
            Err(e) => return Err(format!("Unable to translate because: {}", e)),
        };

        println!("Translation: {}", translation);

        Ok(())
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        if let Err(e) = fs::write("dictionary-save", self.dict.as_json()) {
            println!("Error in saving dictionary: {:?}", e);
        }
    }
}
