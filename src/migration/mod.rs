

#[derive(Default, Debug, Clone)]
pub(crate) enum MigrationType {
    #[default]
    EmotionToScss,
    LegacyToLatest,
    // None,
}