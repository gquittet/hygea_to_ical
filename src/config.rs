#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub postal_code: u16,
}

pub fn parse_config(args: &[String]) -> Config {
    let postal_code = u16::from_str_radix(&args[1], 10).unwrap();

    Config { postal_code }
}
