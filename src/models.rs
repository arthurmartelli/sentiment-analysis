use rust_bert::pipelines::sentiment::{Sentiment, SentimentModel as RustBertSentimentModel};

pub struct SentimentModel {
    pipeline: RustBertSentimentModel,
}

impl SentimentModel {
    pub fn new() -> Self {
        let pipeline = RustBertSentimentModel::new(Default::default())
            .expect("Error initializing sentiment analysis pipeline");

        SentimentModel { pipeline }
    }

    pub fn predict(&mut self, text: &String) -> Vec<Sentiment> {
        let text = [text.as_ref()];

        self.pipeline.predict(text)
    }
}