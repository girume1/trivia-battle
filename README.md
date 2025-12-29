<p align="center">
  <img src="./Trivia on Linera.jpg" width="500">
</p>

## **Trivia Battle - Linera Blockchain DApp**

A multiplayer trivia game built on Linera blockchain with betting, leaderboard, and admin question management.

## **Project Structure**
```
trivia-battle/
├── abi/                # Shared types (questions, leaderboard, etc.)
├── bankroll/           # Token / betting logic
├── trivia/             # Main game contract (rooms, gameplay)
├── master/             # Admin contract (question bank, fees)
├── frontend/           # Flutter web frontend
├── Cargo.toml          # Workspace
└── README.md           # This file
```

## **Features**
- Create/join trivia rooms with bets
- Real-time gameplay
- Global leaderboard
- Admin question management
- Protocol fees collected to treasury

## **Local Development**
**Prerequisites**
- Rust (stable)
- Linera CLI (```cargo install linera-cli```)
- Flutter (for frontend)

## **Build All Contracts**
```
cargo build --release --workspace
```

## **Run Local Network**
```
linera net up --with-faucet
```
Keep this running.

## **Publish Apps (in order)**
```
# In master folder
linera publish-and-create target/wasm32-unknown-unknown/release/master.wasm

# In trivia folder (use master app ID as parameter)
linera publish-and-create target/wasm32-unknown-unknown/release/trivia.wasm --parameters "<master_app_id>"

# In bankroll folder
linera publish-and-create target/wasm32-unknown-unknown/release/bankroll.wasm
```

## **Run Frontend**
```
cd frontend
flutter run -d chrome
```
Open http://localhost:8080 (or port shown)

## **Architecture**
- **master** -> Central question bank + treasury
- **trivia** -> Game rooms and gameplay
- **bankroll** -> Token handling for bets
- **abi** -> Shared types between contracts

## **Commands**
```
# Build all
cargo build --release --workspace

# Clean
cargo clean

# Format
cargo fmt --all
```

## **Future Ideas**
- Daily bonuses
- NFT rewards
- Tournaments
- Mobile app

Happy battling! 🚀💰⚡

Built with ❤️ on **Linera blockchain**
