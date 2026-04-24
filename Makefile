.PHONY: run-engine run-python run-all

run-engine:
	cd engine && cargo run

run-python:
	python3 python/rl_brain.py

run-all:
	@echo "Starting Rust Engine in background..."
	cd engine && cargo run &
	@sleep 5
	@echo "Starting Python RL Brain..."
	python3 python/rl_brain.py
