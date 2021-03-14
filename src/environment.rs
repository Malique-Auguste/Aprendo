use crate::dictionary::Dictionary;
use crate::phrase::{Language, Phrase};
use crate::translation::translate;

use serde::{Deserialize, Serialize};
use std::{io, fs, collections::HashMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    dict: Dictionary,
}

impl Environment {
    pub fn new(path: Option<&str>) -> Result<Environment, String> {
        Ok(Environment {
            dict: Dictionary::new(path)?
        })
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
            Environment::read_input(&mut option);
            let result =  match option.parse() {
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
                    _ => Err("Not a valid option.".into())
                }
                Err(_) => Err(format!("'{}' is not valid input(select a number to begin action).", option))
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

            if phrase == String::from("") {
                break;
            }

            let translation = match translate(&phrase, &input_lang, output_lang, true)
            {
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
        let lang_group: Vec<Language> = lang_group.split("|").map(|s| Language::from_str(s).unwrap()).collect();

        let lang_group_size = self.dict.get_lang_group(&lang_group).len();
        if lang_group_size < 3 {
            return Err("There are less than 3 phrases with that group of language codes.".into())
        }

        let mut test_length = String::new();
        println!("\nYour dictionary currently holds {} phrases with that language code. Enter how many of them you would like to test: ", lang_group_size);
        Environment::read_input(&mut test_length)?;
        let test_length = match test_length.parse() {
            Ok(n) => n,
            Err(e) => return Err(format!("{:?}", e))
        };

        let test_phrases = self.dict.get_test_phrases(test_length, &lang_group);

        println!("Select a, b or c to choose the translation.");
        for phrase in test_phrases.iter() {
            println!("{}", phrase)
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