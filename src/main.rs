mod dictionary;
mod environment;
mod phrase;
mod translation;

fn main() {
    use environment::Environment;
    use translation::translate;
    use phrase::Language;

    println!("{:?}", translate("me llamo malique", &Language::Spanish, Language::English, true))
}
