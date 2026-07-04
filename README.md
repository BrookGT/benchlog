
# benchlog

**benchlog** is a Rust library for the **BLF (Bench Log Format)** used by edge
laboratory instruments to record timestamped multi-channel sensor samples,
discrete events, clock synchronization markers, and run metadata.

BLF targets constrained lab edge devices that capture high-rate analog and
digital channels during instrument runs. The library provides synchronous
parse, validate, incremental stream read, checkpoint resume, run stitching,
and export paths backed by a small allocation-aware memory substrate.

## Features

| module    | role |
|-----------|------|
| `mem`     | refcounted blobs, read cursors, inline scratch, bump slab |
| `frame`   | BLF header, trailer, run metadata sections |
| `channel` | channel descriptors, SI units, linear scaling |
| `sample`  | int/float batch decode, gap detection, interpolation |
| `event`   | markers, alarms, operator annotations |
| `clock`   | sync pulses, monotonic pairing, timestamp correction |
| `codec`   | LEB128, delta-of-deltas packing, zlib stub decode |
| `stream`  | incremental reader with checkpoint resume |
| `stitch`  | merge overlapping instrument runs |
| `export`  | CSV and binary rewrite exporters |
| `check`   | integrity, range, schema validation |
| `ingest`  | top-level parse → validate → report pipeline |

## Layout

```text
benchlog/
  benchlog-core/     # library
  fuzz/              # libFuzzer targets
  .clusterfuzzlite/  # hermetic OSS-Fuzz-style build
  vendor/            # vendored fuzz deps for offline builds
```

## Build

```bash
cargo build --workspace
cargo test --workspace
```

Fuzzing (Linux + nightly + `cargo install cargo-fuzz`):

```bash
cargo fuzz build --offline -O
cargo fuzz run frame_fuzzer
```

See [docs/FORMAT.md](docs/FORMAT.md) for the on-wire BLF layout.

## License

MIT OR Apache-2.0
