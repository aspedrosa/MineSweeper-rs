# Minesweeper - CLI - Rust

My first project in rust.

There are some bug but is playable.
On each play, you must indicate the **mode** and the **coordinates** of the cell to act on, however, on the first play you only need to enter the coordinates.

Modes:
1. `d`: Dig/Show a cell
2. `m`: Mark a cell as a mine
3. `u`: Unmark a cell as a mine

```
play: 3 3  # first play
play: d 3 3  # first play
play: m 3 3  # first play
```


## How to run

1. Generate the binary:
```sh
cargo build --release
```

2. Run:
```sh
./target/release/minesweeper
```

## Generate docs

```sh
cargo doc
```

Then open the index.html at `target/doc/minesweeper/index.html`
