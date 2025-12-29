<p align="center">
  <img src="./Trivia on Linera.jpg" width="500">
</p>
# Trivia Battle ⚡️⛓️

Real-time multiplayer trivia game built on Linera microchains for the Linera Buildathon.

## Features
- Create/join rooms
- Real-time question answering
- Multiplayer state sync
- Bankroll-ready (future betting)
- Flutter web frontend

## Tech Stack
- Backend: Linera SDK (Rust + Wasm)
- Frontend: Flutter Web + GraphQL
- Local testing: Docker Compose

## Setup & Run Locally
1. `cargo build --target wasm32-unknown-unknown --release`
2. `docker compose up` (backend at http://localhost:9002/graphql)
3. In `frontend/`: `flutter pub get && flutter run -d chrome --web-port=8081`

## Live Demo
Frontend: https://trivia-battle.vercel.app  
(Backend GraphQL is local for now — for buildathon demo, deploy to Conway Testnet)

## Buildathon Submission
Wave: [5]  
Category: Games  
Contract: Functional Linera contract with operations for rooms/questions/answers  
Demo: Running on Conway Testnet (link coming soon)

Made with ❤️ for Linera Buildathon 2025
