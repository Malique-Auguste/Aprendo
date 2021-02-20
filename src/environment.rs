use crate::dictionary::Dictionary;
use crate::phrase::{Language, Phrase};
use crate::translation::Translation;

use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Mode {
    Translating,
    Testing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    dict: Dictionary,
    mode: Mode,
}

impl Environment {
    pub fn new_empty() -> Environment {
        Environment {
            dict: Dictionary::new(),
            mode: Mode::Translating,
        }
    }

    pub fn start(&mut self) {
        println!(
            "Welcome to Aperndo!
            \rThis is a command line interace made to handle all your translation needs.\n"
        );

        println!(
            "Select an action.
            \r  1 - Add a single phrase to the dictionary.
            \r  2 - Add multiple phrases to the dictionary.
            \r  3 - Translate a phrase.
            \r  4 - Test a random curation of phrases.
            \r  5 - Test a specific topic.
            \r  6 - Test most difficult phrases.
            \r  7 - HELP!"
        );
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

            let translation = match Translation::translate(&phrase, &input_lang, output_lang, true)
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

        let translation = match Translation::translate(&phrase, &input_lang, output_lang, true) {
            Ok(t) => t,
            Err(e) => return Err(format!("Unable to translate because: {}", e)),
        };

        let phrase = Phrase::new(phrase, input_lang, topic, vec![translation]);

        self.dict.insert(phrase.spelling.clone(), phrase);

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
}