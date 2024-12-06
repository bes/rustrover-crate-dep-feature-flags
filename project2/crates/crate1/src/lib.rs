
pub struct HasFieldsWithFeatureFlags {
    #[cfg(feature = "feat1")]
    pub only_feat_1: bool,
    pub any_feat: bool,
}
