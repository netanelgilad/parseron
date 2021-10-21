use crate::meaning_producer::MeaningProducer;

pub(crate) fn hierarchical_producer(producers: Vec<MeaningProducer>) -> MeaningProducer {
    Box::new(move |context, event| producers[0](context, event))
}

// #[cfg(test)]
// mod tests_2 {
//     use crate::meaning::Meaning;

//     #[test]
//     fn it_works_2() {
//         let producers = vec![
//             Box::new(|context, event| {
//                 if let Meaning::Character = event {
//                     Meaning::Word(String::from("i"))
//                 } else {
//                     Meaning::Empty
//                 }
//             }),
//             Box::new(|context, event| {
//                 if let Meaning::Word(word) = event {
//                     if (word == String::from("i")) {
//                         Meaning::Word(String::from("boo"))
//                     } else {
//                         Meaning::Empty
//                     }
//                 } else {
//                     Meaning::Empty
//                 }
//             })
//         ]
//         assert_eq!(hierarchical_producer(producers), Meaning::Word(String::from("boo")));
//     }
// }
