mod hierarchical_producer;
mod meaning;
mod meaning_producer;

use crate::hierarchical_producer::hierarchical_producer;
use crate::meaning_producer::MeaningProducer;
use meaning::Meaning;
use std::fs::File;
use std::io::{self, BufReader};
use utf8_chars::BufReadCharsExt;

type Matcher = Box<dyn Fn(&Meaning) -> bool>;
type Producer = Box<dyn Fn(Vec<Meaning>) -> Meaning>;

fn identity_matcher(val: Meaning) -> Matcher {
    Box::new(move |event| match event {
        Meaning::Character(char) => match val {
            Meaning::Character(val_char) => *char == val_char,
            _ => false,
        },
        Meaning::Word(word) => match &val {
            Meaning::Word(val_word) => *word == *val_word,
            _ => false,
        },
        _ => false,
    })
}

fn ordered_pattern_producer(matchers: Vec<Matcher>, producer: Producer) -> MeaningProducer {
    Box::new(move |context, event| {
        if matchers[0](&event) {
            producer(vec![event])
        } else {
            context
        }
    })
}

fn producers() -> Vec<MeaningProducer> {
    vec![
        ordered_pattern_producer(
            vec![identity_matcher(Meaning::Character('i'))],
            Box::new(|events| {
                let mut result = String::from("");
                for e in events {
                    match e {
                        Meaning::Character(char) => result.push(char),
                        _ => unreachable!(),
                    }
                }
                Meaning::Word(result)
            }),
        ),
        ordered_pattern_producer(
            vec![identity_matcher(Meaning::Word(String::from("i")))],
            Box::new(move |_| Meaning::Word(String::from("i"))),
        ),
    ]
}

fn main() -> io::Result<()> {
    let f = File::open("foo.txt")?;
    let mut f = BufReader::new(f);

    let mut context = Meaning::Empty;
    let producer = hierarchical_producer(producers());
    for c in f.chars().map(|x| x.unwrap()) {
        context = producer(context, Meaning::Character(c))
    }

    print!("{:?}", context);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
