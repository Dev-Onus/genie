use crate::cli::{derive_options_from_cli_arguments, CliOptions};

mod cli;
mod migration;

fn main() -> std::io::Result<()> {
    let options: CliOptions = derive_options_from_cli_arguments();
    dbg!(&options);

    match options {
        CliOptions::Migration(_m_opts) => {
            println!(
                "\n\n\n********************************\n********START MIGRATION*********\n********************************\n"
            )
        }
        _ => unreachable!(),
    }

    Ok(())
}
