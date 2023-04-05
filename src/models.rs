//! # Transformer neural network
//!
//! The Transformer is a powerful deep learning architecture
//! commonly used in natural language processing tasks such as
//! sentiment analysis. It was first introduced in the paper
//! "Attention Is All You Need" by Vaswani et al. in 2017.
//!
//! The Transformer is a type of neural network architecture
//! that utilizes self-attention mechanisms to process sequential
//! data such as text. It consists of an encoder and decoder,
//! both of which use multi-head attention to selectively focus
//! on different parts of the input sequence.

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
