# Sentiment Analysis with Rust and RustBERT

This is a sample Rust project for performing sentiment analysis on text data using the Rust programming language and the RustBERT library.

## Getting Started

1. Clone the repository:

   ```bash
   git clone https://github.com/arthurmartelli/sentiment_analysis.git
   ```

2. Build and run the project:

   ```bash
   cd sentiment-analysis-rust
   cargo run
   ```

## Usage

This project provides a simple API for performing sentiment analysis on text data. You can use it as follows:

```rust
use sentiment_analysis::SentimentModel;

fn main() {
    let mut model = SentimentModel::new();
    let text = "I love this product!";
    let result = model.predict(&text.to_string());
    println!("Sentiment: {:?}", result);
}
```

Or with the CLI tool:

```bash
sentiment_analysis.exe <TEXT>
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](./LICENSE)
