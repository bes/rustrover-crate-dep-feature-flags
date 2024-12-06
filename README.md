# Proof-of-concept for broken feature flag detection

In `project2/crates/crate1/lib.rs` there is a struct `HasFieldsWithFeatureFlags`
which has a field `only_feat_1` that is only available if the feature
`feat1` is turned on, but RustRover doesn't understand that the feature
isn't turned on in `project1/src/lib.rs` and says 

```
Missing field in `HasFieldsWithFeatureFlags` initializer: `only_feat_1`
```

Which it shouldn't do because the feature isn't turned on.
