
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