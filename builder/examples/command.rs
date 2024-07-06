use derive_builder::Builder;

#[derive(Builder,Debug)]
pub struct Command {
    excutable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(vec![])
        .build()
        .unwrap();
    assert!(command.current_dir.is_none());
}