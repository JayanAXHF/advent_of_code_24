cargo-workspaces workspaces create day_$1 --name day_$1 --lib --edition 2021
filer day_$1/src/bin/part_{1,2}.rs day_$1/src/part_{1,2}.rs
cd day_$1
cargo add tracing tracing-subscriber
sed -i '.bak' '1s/^/pub mod part1;\npub mod part_2;\'$'\n/g' src/lib.rs
