.PHONY: all transport client clean run-server run-client

TRANSPORT_DIR := libtransport
CLIENT_DIR := client
LIB := $(TRANSPORT_DIR)/target/release/libsirfz.so
BIN := $(CLIENT_DIR)/target/release/sirfz
DEST := $(CLIENT_DIR)/target/release

all: transport client
	cp $(LIB) $(DEST)/libsirfz.so
	@echo ""
	@echo "SIRFZ build complete."
	@echo "  Transport:  $(LIB)"
	@echo "  Client: $(BIN)"
	@echo ""
	@echo "Run:"
	@echo "  Server: cd $(DEST) && ./sirfz --server --addr 0.0.0.0:9000"
	@echo "  Client: cd $(DEST) && ./sirfz --addr 127.0.0.1:9000"

transport:
	cd $(TRANSPORT_DIR) && cargo build --release

client:
	cd $(CLIENT_DIR) && cargo build --release

run-server: all
	cd $(DEST) && ./sirfz --server --addr 0.0.0.0:9000 --lib ./libsirfz.so

run-client: all
	cd $(DEST) && ./sirfz --addr 127.0.0.1:9000 --lib ./libsirfz.so

clean:
	cd $(TRANSPORT_DIR) && cargo clean
	cd $(CLIENT_DIR) && cargo clean
