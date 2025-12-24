.PHONY: help run build server-build server server-bg server-stop adder curl-adder clean

ROOT := $(abspath .)
MONGOOSE_DIR := $(ROOT)/web-server/mongoose-master
MONGOOSE_BIN := $(MONGOOSE_DIR)/mongoose
ADDER_SRC := $(MONGOOSE_DIR)/html/cgi-bin/adder.c
ADDER_BIN := $(MONGOOSE_DIR)/html/cgi-bin/adder.exe
SERVER_LOG := /tmp/mongoose.log

UNAME_S := $(shell uname -s)

ifeq ($(UNAME_S),Darwin)
MONGOOSE_MAKE_TARGET := mac
else
MONGOOSE_MAKE_TARGET := linux
endif

help:
	@echo "Targets:"
	@echo "  make build         # cargo build"
	@echo "  make run           # cargo run (littleBrowser)"
	@echo "  make server-build  # build mongoose ($(MONGOOSE_MAKE_TARGET))"
	@echo "  make adder         # build CGI adder.exe"
	@echo "  make server        # run mongoose in foreground"
	@echo "  make server-bg     # run mongoose in background (logs to $(SERVER_LOG))"
	@echo "  make server-stop   # stop process listening on :8080"
	@echo "  make curl-adder    # curl http://127.0.0.1:8080/cgi-bin/adder.exe?1500&212"

build:
	cargo build

run:
	cargo run

server-build:
	$$(/usr/bin/command -v make) -C "$(MONGOOSE_DIR)" $(MONGOOSE_MAKE_TARGET)

adder:
	cc -O2 -o "$(ADDER_BIN)" "$(ADDER_SRC)"
	chmod +x "$(ADDER_BIN)"

server: server-build
	"$(MONGOOSE_BIN)"

server-bg: server-build
	@rm -f "$(SERVER_LOG)"
	@"$(MONGOOSE_BIN)" > "$(SERVER_LOG)" 2>&1 & echo $$! > /tmp/mongoose.pid
	@for i in 1 2 3 4 5 6 7 8 9 10; do \
		lsof -nP -iTCP:8080 -sTCP:LISTEN >/dev/null 2>&1 && break; \
		sleep 0.1; \
	done
	@echo "mongoose started (pid $$(cat /tmp/mongoose.pid)), log: $(SERVER_LOG)"

server-stop:
	@pid=$$(lsof -nP -t -iTCP:8080 -sTCP:LISTEN 2>/dev/null | head -n 1); \
	if [ -n "$$pid" ]; then \
		echo "stopping pid $$pid (port 8080)"; \
		kill $$pid; \
	else \
		echo "no listener on :8080"; \
	fi

curl-adder:
	curl -sS "http://127.0.0.1:8080/cgi-bin/adder.exe?1500&212" | cat

clean:
	cargo clean
	@rm -f "$(SERVER_LOG)" /tmp/mongoose.pid
