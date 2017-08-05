#[macro_use]
extern crate serde_derive;
extern crate docopt;

/// Quartz Command line arguments
pub mod options {
    use docopt::Docopt;

    const USAGE: &'static str = "
    Quartz

    Usage:
      quartz <romfile> [--irate=<r>]
      quartz (-h | --help)

    Options:
      --irate=<r>    Instruction rate [default: 0.0001]
      -h --help      Show help.
    ";

    #[derive(Debug, Deserialize)]
    pub struct ProgramOptions {
        pub arg_romfile: String,
        pub flag_irate: Option<f64>
    }

    pub fn get_program_options() -> ProgramOptions {
        Docopt::new(USAGE).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit())
    }
}
