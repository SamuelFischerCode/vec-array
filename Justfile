commit message :
    cargo test
    cargo clippy -- -Dclippy::all -Dwarnings
    cargo fmt
    git add .
    git commit -m "{{message}}"
    git push

publish version :
    just commit "update version to {{version}} in Cargo.toml"
    cargo publish
