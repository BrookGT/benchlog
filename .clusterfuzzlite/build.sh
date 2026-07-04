
#!/bin/bash -eu
cd "$SRC/benchlog"
cargo fuzz build --offline -O
FUZZ_TARGET_DIR="fuzz/target/x86_64-unknown-linux-gnu/release"
for t in frame_fuzzer channel_fuzzer sample_fuzzer event_fuzzer stream_fuzzer ingest_fuzzer; do
  cp "$FUZZ_TARGET_DIR/$t" "$OUT/"
  if [ -d "fuzz/corpus/$t" ]; then
    (cd "fuzz/corpus/$t" && zip -qr "$OUT/${t}_seed_corpus.zip" .)
  fi
done
