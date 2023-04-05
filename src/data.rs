use rand::seq::SliceRandom;
use rand::thread_rng;
use std::error::Error;

pub struct SentimentData {
    pub inputs: Vec<String>,
    pub labels: Vec<f32>,
    pub batch_counter: usize,
    pub batch_size: usize,
}

impl SentimentData {
    pub fn from_csv(path: &str, batch_size: usize) -> Result<SentimentData, Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b',')
            .from_path(path)?;

        let mut inputs = Vec::new();
        let mut labels = Vec::new();

        for result in rdr.records() {
            let record = result?;
            let label = match &record[2] {
                "positive" => 1.0,
                "negative" => -1.0,
                _ => 0.0,
            };
            let input = record[3].to_owned();

            labels.push(label);
            inputs.push(input);
        }

        Ok(Self {
            inputs,
            labels,
            batch_size,
            batch_counter: 0,
        })
    }

    pub fn len(&self) -> usize {
        self.inputs.len()
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();

        let mut perm: Vec<_> = (0..self.inputs.len()).collect();
        perm.shuffle(&mut rng);

        self.inputs = perm.iter().map(|&i| self.inputs[i].clone()).collect();
        self.labels = perm.iter().map(|&i| self.labels[i]).collect();
    }

    pub fn next_batch(&mut self, batch_size: usize) -> Option<SentimentData> {
        let start = self.batch_counter;
        let end = std::cmp::min(self.inputs.len(), start + batch_size);

        if start >= end {
            return None;
        }

        let inputs = self.inputs[start..end].to_vec();
        let labels = self.labels[start..end].to_vec();
        let mut data = Self {
            inputs,
            labels,
            batch_counter: self.batch_counter,
            batch_size,
        };

        data.shuffle();
        self.batch_counter += batch_size;

        Some(data)
    }
}

impl Iterator for SentimentData {
    type Item = SentimentData;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_batch(self.batch_size)
    }
}
