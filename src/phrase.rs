use crate::translation::Translation;

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Language {
    English,
    French,
    Spanish,
}

impl Language {
    pub fn from_str(input: &str) -> Result<Language, String> {
        match input {
            "en" | "english" | "English" | "ENGLISH" => Ok(Language::English),
            "fr" | "french" | "French" | "FRENCH" => Ok(Language::French),
            "es" | "spanish" | "Spanish" | "SPANISH" => Ok(Language::Spanish),
            _ => Err(format!(
                "{} is not a language translatable by this program.",
                input
            )),
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::English => write!(f, "en"),
            Language::French => write!(f, "fr"),
            Language::Spanish => write!(f, "es"),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub enum Difficulty {
    VeryEasy,
    Easy,
    Average,
    Hard,
    VeryHard,
}

impl Difficulty {
    pub fn as_num(&self) -> usize {
        match self {
            Difficulty::VeryEasy => 1,
            Difficulty::Easy => 4,
            Difficulty::Average => 9,
            Difficulty::Hard => 16,
            Difficulty::VeryHard => 25,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Phrase {
    pub spelling: String,
    pub language: Language,

    pub translations: Vec<Translation>,
    pub topic: String,

    pub last_tested: SystemTime,
    pub difficulty: Difficulty,
    pub length: usize,
}

impl Phrase {
    pub fn new<S: Into<String>>(
        spelling: S,
        language: Language,
        topic: String,
        translations: Vec<Translation>,
    ) -> Phrase {
        let spelling: String = spelling.into();
        let length = spelling.chars().count();

        Phrase {
            spelling,
            language,
            translations,
            topic,
            last_tested: SystemTime::now(),
            difficulty: Difficulty::Average,
            length,
        }
    }
}

impl std::fmt::Display for Phrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.spelling)
    }
}

impl std::fmt::Debug for Phrase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            return write!(f, "Spelling: {}\nLanguage: {:?}\nTranslations: {:?}\nTopic: {}\nLast Tested: {:?}\nDifficulty: {:?}\nLength {}", self.spelling, self.language, self.translations, self.topic, self.last_tested, self.difficulty, self.length);
        } else {
            return write!(
                f,
                "Spelling: {}, Language: {}, Translations: {:?}, Topic: {}",
                self.spelling, self.language, self.translations, self.topic
            );
        }
    }
}
