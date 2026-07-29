# micompress (Rust)

> Compress **images, PDFs, video, audio, documents & archives** from the command line.
> Powered by [**micompress.com**](https://micompress.com/?utm_source=crates&utm_medium=cli&utm_campaign=extend).

```bash
cargo install micompress
micompress photo.jpg
```

## Usage

```bash
micompress *.png --preset extreme
micompress report.pdf -o out/
```

Options: `-o/--output`, `-p/--preset` (`lossless|recommended|extreme`), `--token`, `--api`, `-q/--quiet`.

## Build & publish

Requires the Rust toolchain (not available on this build server):

```bash
cargo build --release
cargo publish
```

## License

MIT © [MiCompress](https://micompress.com)
