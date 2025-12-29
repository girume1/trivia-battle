#!/bin/bash
set -e

echo "Starting Linera network..."
linera net up --with-faucet &

sleep 10

echo "Starting frontend..."
cd frontend/build/web
http-server -p 3000 &

echo "Trivia Battle ready!"
echo "Frontend: http://localhost:3000"
echo "Linera: http://localhost:8080"

wait