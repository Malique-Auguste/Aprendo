mod dictionary;
mod environment;
mod helper;
mod phrase;
mod translation;

fn main() {
    use environment::Environment;

    let mut env = Environment::new(Some("dictionary-save")).unwrap();
    env.start();
}
