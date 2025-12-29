# Trivia Battle Makefile
# Use: make build, make run, make clean

.PHONY: all build publish run frontend clean

all: build publish run

build:
	@echo "Building all apps..."
	cd bankroll && cargo build --release
	cd trivia && cargo build --release
	cd master && cargo build --release

publish:
	@echo "Publishing apps..."
	@linera wallet init --with-new-chain --faucet http://localhost:8080 || true
	$(eval CHAIN_ID := $(shell linera wallet show | grep "Default chain" | awk '{print $$3}'))
	@echo "Using chain: $(CHAIN_ID)"
	
	@echo "Publishing Bankroll..."
	$(eval BANKROLL := $(shell linera publish-and-create bankroll/target/wasm32-unknown-unknown/release/bankroll.wasm))
	@echo "Bankroll: $(BANKROLL)"
	
	@echo "Publishing Trivia..."
	$(eval TRIVIA := $(shell linera publish-and-create trivia/target/wasm32-unknown-unknown/release/trivia.wasm --parameters "$(CHAIN_ID) $(BANKROLL)"))
	@echo "Trivia: $(TRIVIA)"
	
	@echo "Publishing Master..."
	$(eval QUESTIONS := '[{"id":1,"text":"What is 2+2?","choices":["3","4","5","6"],"correct_idx":1,"category":"Math","difficulty":1}]')
	$(eval MASTER := $(shell linera publish-and-create master/target/wasm32-unknown-unknown/release/master.wasm --parameters "$(QUESTIONS) $(CHAIN_ID)"))
	@echo "Master: $(MASTER)"

run:
	@echo "Starting Linera server..."
	linera-server run & echo $$! > linera.pid
	sleep 5

frontend:
	@echo "Starting Flutter frontend..."
	cd frontend && flutter pub get && flutter run -d chrome

clean:
	@echo "Cleaning..."
	kill $$(cat linera.pid) || true
	rm -f linera.pid
	cd bankroll && cargo clean
	cd trivia && cargo clean
	cd master && cargo clean