use khonsu_tools::{
    universal::{
        anyhow,
        clap::Parser,
        code_coverage::{self},
        DefaultConfig,
    },
    Commands,
};

fn main() -> anyhow::Result<()> {
    Commands::parse().execute::<Config>()
}

struct Config;

impl khonsu_tools::Config for Config {
    type Publish = Self;
    type Universal = Self;
}

impl khonsu_tools::universal::Config for Config {
    type Audit = DefaultConfig;
    type CodeCoverage = Self;
}

impl code_coverage::Config for Config {
    fn ignore_paths() -> Vec<String> {
        vec![String::from("actionable/examples/*")]
    }
}

impl khonsu_tools::publish::Config for Config {
    fn paths() -> Vec<String> {
        vec![
            String::from("actionable-macros"),
            String::from("actionable"),
        ]
    }
}
