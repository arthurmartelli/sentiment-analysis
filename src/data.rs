pub struct SentimentData {
    path: String,
}

impl SentimentData {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}
