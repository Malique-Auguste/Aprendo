mod dictionary;
mod environment;
mod phrase;
mod translation;
mod helper;

fn main() {
    use environment::Environment;

    let mut env = Environment::new(Some("dictionary-save")).unwrap();
    env.start();
}