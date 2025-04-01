cargo-workspaces workspaces create day_$1 --name day_$1 --lib --edition 2021
cd day_$1
filer src/bin/part_{1,2}.rs src/part_{1,2}.rs
cargo add tracing tracing-subscriber
sed -i '.bak' '1s/^/pub mod part1;\npub mod part_2;\'$'\n/g' src/lib.rs
