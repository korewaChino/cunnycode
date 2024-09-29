use clap::{Parser, ValueEnum};
use clap_stdin::MaybeStdin;

const DOT: char = '😭';
const DASH: char = '💢';

fn into_ccode(input: &str) -> String {
    format!(
        "^{}",
        crypto_morse::encode(input)
            .chars()
            .map(|c| match c {
                '.' => DOT,
                '_' => DASH,
                _ => c,
            })
            .collect::<String>()
            .replace("/ ", "^")
    )
}

fn from_ccode(input: &str) -> String {
    let i = input
        .trim_start_matches("^")
        .replace("^", "/ ")
        .replace("  ", " / ")
        .replace(DOT, ".")
        .replace(DASH, "_")
        .chars()
        // filter out any characters that are not morse code
        .filter(|c| c == &'.' || c == &'_' || c == &' ' || c == &'/')
        .collect::<String>();

    println!("{}", i);

    crypto_morse::decode(&i)
}

#[derive(ValueEnum, Debug, Clone)]
enum Mode {
    To,
    From,
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(value_enum, short, long)]
    mode: Mode,
    input: MaybeStdin<String>,
}
fn main() {
    let args = Cli::parse();

    match args.mode {
        Mode::To => {
            println!("{}", into_ccode(&args.input));
        }
        Mode::From => {
            println!("{}", from_ccode(&args.input));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proper_conversion() {
        let input = "hello world!!!";
        let ccode = into_ccode(input);
        let output = from_ccode(&ccode);
        assert_eq!(input, output);
    }
}
