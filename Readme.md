# Genie Code Mod Tool

## way to run

### mac 

```sh
clear && cargo run -- migrate ejs-scss -I /Users/USERNAME/MyWorkspace/JS/Monthly-Cart/monthly-cart-app/src/index.jsx
```

### windows

```sh
cls && cargo run -- migrate ejs-scss -I c:/MyWorkspace/JS/Monthly-Cart/monthly-cart-app/src/index.jsx
```

### Output to be seen

```sh
warning: `genie` (bin "genie") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/genie migrate ejs-scss -I /Users/USERNAME/MyWorkspace/JS/Monthly-Cart/monthly-cart-app/src/index.jsx`
[src/main.rs:8] &options = Migration(
    MigrationOptions {
        command: EmotionToScss,
        input: File {
            path: "/Users/USERNAME/MyWorkspace/JS/Monthly-Cart/monthly-cart-app/src/index.jsx",
            is_file: true,
        },
        output: None,
        alias: None(
            "/Users/USERNAME/MyWorkspace/JS/Monthly-Cart/monthly-cart-app",
        ),
    },
)



********************************
********START MIGRATION*********
********************************
```
