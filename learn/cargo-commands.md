# cargo commands to remember

## cargo add

provide all the dependencies needed one by one separated by space and for features within them, use features option by prefixing the crate name before the feature needed.

```shellscript
cargo add clap serde_json  serde --features serde/derive
```
