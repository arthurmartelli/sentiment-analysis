extern crate clap;

use clap::Parser;

mod args;

use sentiment_analysis::SentimentModel;

fn main() {
    let args = args::EvalOptions::parse();
    command_handler(args);
}

fn command_handler(args: args::EvalOptions) {
    let input = args.text;
    let mut model = SentimentModel::new();
    let prediction = model.predict(&input);

    let polarity = &prediction[0].polarity;
    let score = prediction[0].score * 100 as f64;

    println!("Prediction: {polarity:?} [{score:3.3}%]");
}
