
# BLF — Bench Log Format

BLF stores laboratory instrument capture streams as an append-friendly binary
log. Each file begins with a fixed header, followed by typed sections, and ends
with a trailer carrying a rolling CRC and byte count.

## Header (32 bytes)

| offset | size | field |
|--------|------|-------|
| 0 | 5 | magic `BLF\x01` |
| 5 | 1 | flags |
| 6 | 2 | format version (LE u16) |
| 8 | 8 | run start unix ns (LE u64) |
| 16 | 8 | instrument id (LE u64) |
| 24 | 4 | section count (LE u32) |
| 28 | 4 | header CRC32 (LE u32) |

## Section envelope

Each section is prefixed with:

- `kind` — u8 section type
- `flags` — u8
- `length` — LEB128 payload byte count
- `payload` — typed body

Section kinds:

| code | name |
|------|------|
| 1 | RUN_INFO |
| 2 | CHANNEL_MAP |
| 3 | SAMPLE_BLOCK |
| 4 | EVENT_BLOCK |
| 5 | CLOCK_SYNC |
| 6 | CHECKPOINT |

## Channel descriptor

A channel entry encodes:

- channel id (LE u32)
- sample kind (u8): 0=int32, 1=float32, 2=bool pack
- unit code (LE u16, see `channel/units.rs`)
- scale (LE f64) and offset (LE f64)
- name (length-prefixed UTF-8)

## Sample block

Sample blocks batch values for one channel:

- base timestamp ns (LE u64)
- sample count (LE u32)
- encoding (u8): 0=raw, 1=delta, 2=delta-of-deltas
- payload bytes

## Event block

Events carry a timestamp, severity u8, kind u8, and optional UTF-8 note.

## Clock sync block

Sync blocks pair a host monotonic counter with an instrument tick counter for
post-hoc timestamp alignment.

## Trailer (16 bytes)

- total payload bytes (LE u64)
- rolling CRC32 (LE u32)
- reserved (LE u32)
