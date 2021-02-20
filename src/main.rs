mod dictionary;
mod environment;
mod phrase;
mod translation;

fn main() {
    use environment::Environment;

    let mut env = Environment::new_empty();
    env.start();
}
