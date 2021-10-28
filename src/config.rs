use clap::{crate_authors, crate_description, crate_version, Parser};

#[derive(Debug, Parser, Clone)]
#[clap(
    author = crate_authors!(),
    about = crate_description!(),
    version = crate_version!(),
)]
pub struct Config {
    pub postal_code: u16,
    pub street: Option<String>,
}
