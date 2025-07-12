#!/bin/sh
RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo build
