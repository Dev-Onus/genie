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