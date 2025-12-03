use crate::domain::node_garden::NodeGarden;
use crate::domain::vaam::VocabWord;

pub fn get_node_gardens() -> Vec<NodeGarden> {
    vec![NodeGarden::new(
        "NG_BELL_TOWER".to_string(),
        "The Bell Tower".to_string(),
        40.4275,  // Purdue Bell Tower Lat
        -86.9136, // Purdue Bell Tower Lon
        50.0,     // 50 meter radius
        "The Bell Tower".to_string(),
    )]
}

pub fn get_vocab_words() -> Vec<VocabWord> {
    vec![VocabWord {
        id: 1,
        word: "Campanile".to_string(),
        definition: "A freestanding bell tower, especially of Italian design.".to_string(),
        context_tag: Some("The Bell Tower".to_string()),
        complexity_tier: Some(2),
    }]
}
