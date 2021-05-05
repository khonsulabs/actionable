#!/bin/bash
set -e
echo "Executing tests"
cargo clean
CARGO_INCREMENTAL=0 LLVM_PROFILE_FILE="%m.profraw" RUSTFLAGS="-Zinstrument-coverage" RUSTDOCFLAGS="-Cpanic=abort" cargo +nightly test --all-features -- --nocapture
echo "Generating coverage report"
grcov . --binary-path ./target/debug/ -s . -t html --branch --ignore-not-existing --llvm --ignore "target/*" --ignore "actionable/examples/*" -o coverage/
COVERAGE=`cat coverage/index.html | grep -oP '\d+(\.\d+)? %' | head -n1`
if command -v cargo-badge &> /dev/null
then
    echo "Generating badge"
    cargo badge -s "coverage" "$COVERAGE" -o coverage/badge.svg
fi
echo "::warning::Line Coverage Percentage: $COVERAGE"
echo "Cleaning up."
find . -name "*.profraw" -exec rm {} \;