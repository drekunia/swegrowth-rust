use std::io::{self, Write};


#[derive(Debug, PartialEq)]
pub enum CaseMode {
    CaseSensitive,
    CaseInsensitive
}

pub fn parse_args(args: &[String]) -> Result<CaseMode, String> {
    if args.len() <= 1 {
        return Ok(CaseMode::CaseSensitive);
    }

    if args.len() > 2 {
        return Err(
            "Too many arguments. Only --case-sensitive or --case-insensitive allowed.".to_string(),
        );
    }

    match args[1].as_str() {
        "--case-sensitive" => Ok(CaseMode::CaseSensitive),
        "--case-insensitive" => Ok(CaseMode::CaseInsensitive),
        invalid_arg => Err(format!(
            "Unknown argument '{}'. Valid options are --case-sensitive or --case-insensitive.",
            invalid_arg
        )),
    }
}

pub fn prompt_input(mode: &CaseMode) -> io::Result<String> {
    print!("Enter letters ({:?}): ", mode);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
