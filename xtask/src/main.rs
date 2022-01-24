use khonsu_tools::{
    publish,
    universal::{
        anyhow,
        clap::Parser,
        code_coverage::{self},
        DefaultConfig,
    },
};

fn main() -> anyhow::Result<()> {
    khonsu_tools::Commands::parse().execute::<Config>()
}

struct Config;

impl khonsu_tools::universal::Config for Config {
    type Audit = DefaultConfig;

    type CodeCoverage = Self;
}

impl khonsu_tools::Config for Config {
    type Publish = Self;

    type Universal = Self;
}

impl code_coverage::Config for Config {
    fn ignore_paths() -> Vec<String> {
        vec![String::from("actionable/examples/*")]
    }
}

impl publish::Config for Config {
    fn paths() -> Vec<String> {
        vec![
            String::from("actionable-macros"),
            String::from("actionable"),
        ]
    }
}
