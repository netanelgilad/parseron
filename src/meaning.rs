#[derive(Debug)]
pub enum Meaning {
    Empty,
    Character(char),
    Word(String),
}
