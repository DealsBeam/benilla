# benilla-xess

A production-safe add-on layer for benilla that enables Intel XeSS-style rendering through the supported extension pattern: build on top of `benilla_app::run_with`, never inside the stock benilla core.

## Why this exists

The project explicitly treats modern or non-vanilla features as crate-level extensions rather than core changes. This crate follows that rule and provides a clean integration point for XeSS-like upscaling.

## How to use

Set the add-on runtime with environment variables:

```bash
BENILLA_XESS=1
BENILLA_XESS_MODE=quality
BENILLA_XESS_SCALE=1.5
```

Then launch the add-on:

```bash
cargo run -p benilla-xess --example xess_launcher
```

## Production notes

This crate is designed as a clean hook for a real Intel XeSS runtime integration. For actual hardware upscaling, it should be wired to the official XeSS SDK or a maintained bridge such as the ponytail.dev stack when available.
