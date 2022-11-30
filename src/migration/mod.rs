use std::path::PathBuf;
use serde_json::{Value};

#[derive(Default, Debug, Clone)]
pub(crate) enum MigrationType {
    #[default]
    EmotionToScss,
    LegacyToLatest,
    // None,
}

#[derive(Debug, Clone)]
pub(crate) enum MigrationImportAliasType {
    TypeScript(Value, PathBuf),
    JavaScript(Value, PathBuf),
    None(PathBuf),
}

#[derive(Debug, Clone)]
pub(crate) enum MigrationPathMetaData {
    File { path: PathBuf, is_file: bool },
    Directory { path: PathBuf, is_directory: bool },
    Error(String),
    None,
}

#[derive(Debug, Clone)]
pub(crate) struct MigrationOptions {
    command: MigrationType,
    input: MigrationPathMetaData,
    output: MigrationPathMetaData,
    alias: MigrationImportAliasType,
}