use clap::{Arg, ArgAction, Command};

use crate::migration::MigrationOptions;

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

fn build_command_line() -> Command {
    Command::new("genie")
        .about("Command line tool to migrate code in your react project")
        .version("0.0.1")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(create_migration_command())
}

fn create_migration_command() -> Command {
    Command::new("migrate")
        .arg_required_else_help(true)
        .subcommand(create_ejs_to_scss_subcommand())
        .subcommand(create_legacy_to_latest_subcommand())
}

fn create_ejs_to_scss_subcommand() -> Command {
    Command::new("ejs-scss")
        .arg_required_else_help(true)
        .arg(create_input_argument())
        .arg(create_output_argument())
        .arg(create_js_alias_argument())
}

fn create_legacy_to_latest_subcommand() -> Command {
    Command::new("legacy")
        .arg_required_else_help(true)
        .arg(create_input_argument())
        .arg(create_output_argument())
        .arg(create_js_alias_argument())
}

fn create_input_argument() -> Arg {
    Arg::new("input")
        .short('I')
        .long("input-file")
        .action(ArgAction::Set)
        .help("entry file of the repo")
        .value_name("FILE|DIRECTORY")
}

fn create_output_argument() -> Arg {
    Arg::new("output")
        .short('O')
        .long("output-file")
        .action(ArgAction::Set)
        .help("target file to be generated")
        .value_name("FILE|DIRECTORY")
}

fn create_js_alias_argument() -> Arg {
    Arg::new("js_alias")
        .short('A')
        .long("js-config-alias")
        .action(ArgAction::SetTrue)
        .help("is js/ts config aliases enabled in the repo?")
        .value_name("BOOLEAN")
}
