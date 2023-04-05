extern crate clap;

use clap::Parser;

mod args;
mod models;

pub use models::SentimentModel;

fn main() {
    let input = args::EvalOptions::parse().text;
    let mut model = SentimentModel::new();
    let prediction = model.predict(&input);

    let polarity = &prediction[0].polarity;
    let score = prediction[0].score * 100 as f64;

    println!("Prediction: {polarity:?} [{score:3.3}%]");
}
