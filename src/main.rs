extern crate clap;

use clap::Parser;

mod args;
mod data;
mod models;

pub use data::SentimentData;
pub use models::SentimentModel;

fn main() {
    let args = args::EvalOptions::parse();

    let input = args.text;

    let mut model = get_module(args.new_model);

    let save_path = args.save_path;
    if save_path != String::new() {
        println!("Saving model at {save_path}");
        model.save(save_path)
    }

    let prediction = model.predict(&input);
    println!("Prediction: {prediction}");
}

fn get_module(arg_new_module: bool) -> SentimentModel {
    let mut model = SentimentModel::new();

    if arg_new_module {
        println!("Creating a new model");

        let training_data = SentimentData::new("data/twitter_training.csv".to_string());
        let validate_data = SentimentData::new("data/twitter_validate.csv".to_string());

        model.train(&training_data);
        model.eval(&validate_data);
    } else {
        model.load_pretrained();
    }

    model
}
