use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct EvalOptions {
    /// String to evaluate
    #[clap(short, long, required = true)]
    pub text: String,

    /// If the model needs to be new
    #[clap(short, long, default_value_t = false)]
    pub new_model: bool,

    #[clap(short, long, default_value_t = String::new())]
    pub save_path: String,
}
