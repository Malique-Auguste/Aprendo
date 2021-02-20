use crate::translation::Translation;

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Language {
    English,
    French,
    Spanish,
}

impl Language {
    pub fn from_str(input: &str) -> Result<Language, ()> {
        match input {
            "en" | "english" | "English" | "ENGLISH" => Ok(Language::English),
            "fr" | "french" | "French" | "FRENCH" => Ok(Language::French),
            "es" | "spanish" | "Spanish" | "SPANISH" => Ok(Language::Spanish),
            _ => Err(()),
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Phrase {
    pub spelling: String,
    pub language: Language,

    pub translations: Vec<Translation>,
    pub topic: String,

    last_tested: SystemTime,
    score: i8,
}

impl Phrase {
    pub fn new<S: Into<String>>(
        spelling: S,
        language: Language,
        topic: String,
        translations: Vec<Translation>,
    ) -> Phrase {
        Phrase {
            spelling: spelling.into(),
            language,
            translations,
            topic,
            last_tested: SystemTime::now(),
            score: 50,
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
            return write!(f, "Spelling: {}\nLanguage: {:?}\nTranslations: {:?}\nTopic: {}\nLast Tested: {:?}\nScore: {}", self.spelling, self.language, self.translations, self.topic, self.last_tested, self.score);
        } else {
            return write!(
                f,
                "Spelling: {}, Language: {}, Translations: {:?}, Topic: {}",
                self.spelling, self.language, self.translations, self.topic
            );
        }
    }
}
