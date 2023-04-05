use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct EvalOptions {
    /// String to evaluate
    #[clap(required = true)]
    pub text: String,
}
