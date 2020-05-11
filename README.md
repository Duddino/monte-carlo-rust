# monte-carlo-rust
Visualization of Monte Carlo made with amethyst

## How to install

Clone the repository.

To run the game, run the following command on the directory where you cloned the repository, which defaults to the `vulkan` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --no-default-features --features "metal"
```
To run the game with optimazations use the following command:
```bash
cargo run --features "fast" --release
```
