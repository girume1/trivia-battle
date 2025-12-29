#!/bin/bash

# Trivia Battle Full Launch Script
# Runs Linera node, builds & publishes apps, starts frontend

set -e  # Stop on error

echo "=== Starting Linera local node ==="
linera-server run &

sleep 5  # Wait for server

echo "=== Creating wallet & chains ==="
linera wallet init --with-new-chain --faucet http://localhost:8080

# Get default chain ID
CHAIN_ID=$(linera wallet show | grep "Default chain" | awk '{print $3}')

echo "Default chain: $CHAIN_ID"

echo "=== Building & publishing Bankroll app ==="
(cd bankroll && cargo build --release)
BANKROLL_APP=$(linera publish-and-create bankroll/target/wasm32-unknown-unknown/release/bankroll.wasm)

echo "Bankroll app ID: $BANKROLL_APP"

echo "=== Building & publishing Trivia app ==="
(cd trivia && cargo build --release)
TRIVIA_APP=$(linera publish-and-create trivia/target/wasm32-unknown-unknown/release/trivia.wasm --parameters "$CHAIN_ID $BANKROLL_APP")

echo "Trivia app ID: $TRIVIA_APP"

echo "=== Building & publishing Master app ==="
(cd master && cargo build --release)
QUESTIONS='[{"id":1,"text":"Capital of France?","choices":["London","Paris","Berlin","Madrid"],"correct_idx":1,"category":"Geography","difficulty":1}]'  # Add more later
MASTER_APP=$(linera publish-and-create master/target/wasm32-unknown-unknown/release/master.wasm --parameters "$QUESTIONS $CHAIN_ID")

echo "Master app ID: $MASTER_APP"

echo "=== Starting Frontend ==="
(cd frontend && flutter pub get && flutter run -d chrome)

echo "=== All done! Trivia Battle is running ==="