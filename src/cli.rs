use clap::{Arg, ArgAction};


use crate::migration::{
    MigrationOptions,
};

#[derive(Debug, Clone)]
enum PathType {
    Output,
    Input,
}

#[derive(Debug, Clone)]
pub(crate) enum CliOptions {
    Migration(MigrationOptions),
    Error(String),
    None,
}

fn create_js_alias_argument() -> Arg {
    Arg::new("js_alias")
        .short('A')
        .long("js-config-alias")
        .action(ArgAction::SetTrue)
        .help("is js/ts config aliases enabled in the repo?")
        .value_name("BOOLEAN")
}