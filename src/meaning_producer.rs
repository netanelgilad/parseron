use crate::meaning::Meaning;

pub type MeaningProducer = Box<dyn Fn(Meaning, Meaning) -> Meaning>;
