use crate::SentimentData;

pub struct SentimentModel {}

impl SentimentModel {
    pub fn new() -> Self {
        todo!()
    }

    pub fn train(&mut self, data: &SentimentData) {
        todo!()
    }

    pub fn eval(&mut self, data: &SentimentData) {
        todo!()
    }

    pub fn predict(&mut self, text: &String) -> String {
        todo!()
    }

    pub fn save(&self, path: String) {
        todo!()
    }

    pub fn load_pretrained(&mut self) {
        todo!()
    }
}
