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

Options: `-o/--output`, `-p/--preset` (`lossless|recommended|extreme`), `--api-key` (personal API key, see below), `--token` (legacy login token), `--suffix`, `--api`, `-q/--quiet`.

## API key

Without a key the CLI works anonymously, with small file-size limits. For larger files and a
monthly allowance, pass your personal API key (`mc_…`) — sign up for free and copy it from
[micompress.com/account](https://micompress.com/account).

```bash
micompress --api-key mc_xxx photo.jpg
# or
export MICOMPRESS_API_KEY=mc_xxx
micompress photo.jpg
```

`--api-key` takes precedence over `MICOMPRESS_API_KEY`. The key is sent as an `X-API-Key` header on
the compress request only.

| Plan | Compressions per month |
| --- | --- |
| Anonymous (no key) | works, small file-size limits |
| Free account | 50 via the API, files up to 100 MB |
| Plus ($9.99/mo) | 1,000 (shared with website usage) |
| Pro ($19.99/mo) | 5,000 (shared with website usage) |

When the allowance is used up the CLI prints the server's message (`API_QUOTA_REACHED` for the free
key, `USAGE_LIMIT_REACHED` for paid plans). See [micompress.com/plans](https://micompress.com/plans).

The older `--token <jwt>` (login token) option is still supported.

## Build & publish

Requires the Rust toolchain:

```bash
cargo build --release
cargo publish
```

## License

MIT © [MiCompress](https://micompress.com)
