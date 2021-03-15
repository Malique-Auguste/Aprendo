use crate::phrase::Language;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Translation {
    pub spelling: String,
    pub language: Language,
}

impl Translation {
    pub fn new<S: Into<String>>(language: Language, spelling: S) -> Translation {
        Translation {
            spelling: spelling.into(),
            language,
        }
    }
}

impl std::fmt::Display for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.spelling)
    }
}

impl std::fmt::Debug for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            return write!(
                f,
                "Spelling: {}\nLanguage: {:?}",
                self.spelling, self.language
            );
        } else {
            return write!(
                f,
                "Spelling: {}, Language: {}",
                self.spelling, self.language
            );
        }
    }
}

pub fn translate(
    phrase: &str,
    input_lang: &Language,
    output_lang: Language,
    show_output: bool,
) -> Result<Translation, String> {
    let url = format!(
        "https://api.mymemory.translated.net/get?q={}&langpair={}|{}",
        phrase, input_lang, output_lang
    );

    if show_output {
        println!("translating...");
    }

    let response = reqwest::blocking::get(&url).unwrap().text().unwrap();

    if !response.contains("\"responseStatus\":200,\"") {
        return Err(format!(
            "Response doesnt have a response status of 200 (OK). Response is:\n {}",
            response
        ));
    }

    let mut num = 0;
    let mut translated = String::new();

    for c in response.chars() {
        if c == '"' {
            if num > 4 {
                break;
            }
            num += 1;
        } else if num == 5 {
            translated.push(c)
        }
    }

    Ok(Translation::new(output_lang, translated))
}
