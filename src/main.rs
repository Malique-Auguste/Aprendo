mod dictionary;
mod environment;
mod helper;
mod phrase;
mod translation;

use phrase::Language;
use translation::translate;

use std::env;

fn main() {
    use environment::Environment;
    println!();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let mut env = match Environment::new(Some("dictionary-save")) {
            Ok(env) => env,
            Err(e) => {
                println!("Err: {}\n", e.1);
                e.0
            }
        };
        env.start();
    } else if args.len() == 4 {
        let phrase = &args[1].replace('_', " ");

        let input_lang = match Language::from_str(&args[2]) {
            Ok(l) => l,
            Err(e) => {
                println!("Err: {:?}", e);
                return;
            }
        };
        let output_lang = match Language::from_str(&args[3]) {
            Ok(l) => l,
            Err(e) => {
                println!("Err: {:?}", e);
                return;
            }
        };

        println!(
            "{}",
            match translate(phrase, &input_lang, output_lang, false) {
                Ok(t) => t.spelling,
                Err(e) => format!("Err: {}", e),
            }
        )
    } else {
        println!(
            "To translate directly, arguments must follow the following format:
        \r<phrase> <input language> <output language>
        
        \r(words in phrases must be separated by an underscore as opposed to a space)"
        );
    }

    println!();
}
