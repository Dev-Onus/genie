use std::{path::{PathBuf, Path}, fs};

use clap::{Arg, ArgAction, Command};
use serde_json::Value;

use crate::migration::{MigrationOptions, MigrationPathMetaData};

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

fn search_package_json(given_path: &Path, is_file: bool) -> Option<PathBuf> {
    let given_directory = if is_file {
        given_path.parent().unwrap()
    } else {
        given_path
    };

    // dbg!(&given_path);

    let mut project_directory = Some(given_directory.clone());
    let mut found_root = false;

    while let Some(current_path) = project_directory {
        // dbg!(current_path);
        if current_path.join("package.json").try_exists().unwrap() {
            // project_directory = current_path;
            found_root = true;
            break
        } else {
            project_directory = current_path.parent()
        }
    }

    // dbg!(&project_directory);

    if found_root {
        match project_directory {
            Some(path) => Some(path.to_path_buf()),
            None => None,
        }
    } else {
        None
    }
}

fn get_js_config_value_from_path(jsc: PathBuf) -> Value {
    let data = fs::read_to_string(jsc).unwrap_or_else(|_| String::from("{}"));
    let v: Value = serde_json::from_str(&data).unwrap();
    // dbg!(&v);
    v["compilerOptions"]["paths"].clone()
}

fn check_if_path_exists(given_path: Option<&String>, path_type: PathType) -> MigrationPathMetaData {
    if let Some(output_file) = given_path {
        let path = PathBuf::from(&output_file);

        match path.try_exists() {
            Ok(is_exists) => {
                if is_exists {
                    if path.is_file() {
                        MigrationPathMetaData::File {
                            path: path,
                            is_file: true,
                        }
                    } else if path.is_dir() {
                        MigrationPathMetaData::Directory {
                            path: path,
                            is_directory: true,
                        }
                    } else {
                        MigrationPathMetaData::None
                    }
                } else {
                    let error_string = match path_type {
                        PathType::Output => String::from(
                            r#"output path is optional. While given it is expected to be a valid path. Remove output path if you want the input file to be rewritten."#,
                        ),
                        PathType::Input => String::from(
                            r#"input path is not optional. try again by providing a valid system path"#,
                        ),
                    };

                    MigrationPathMetaData::Error(error_string)
                }
            }
            _ => MigrationPathMetaData::Error(String::from(
                r#"Unable to verify if the given file exists, please try again."#,
            )),
        }
    } else {
        MigrationPathMetaData::None
    }
}

fn get_cli_arguments_matches() -> clap::ArgMatches {
    build_command_line().get_matches()
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
