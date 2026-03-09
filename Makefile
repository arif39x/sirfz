.PHONY: all transport hypervisor clean run-server run-client

TRANSPORT_DIR := transport
HYPERVISOR_DIR := hypervisor
LIB := $(TRANSPORT_DIR)/libsirfz.so
BIN := $(HYPERVISOR_DIR)/target/release/sirfz
DEST := $(HYPERVISOR_DIR)/target/release

all: transport hypervisor
	cp $(LIB) $(DEST)/libsirfz.so
	@echo ""
	@echo "SIRFZ build complete."
	@echo "  Transport:  $(LIB)"
	@echo "  Hypervisor: $(BIN)"
	@echo ""
	@echo "Run:"
	@echo "  Server: cd $(DEST) && ./sirfz --server --addr 0.0.0.0:9000"
	@echo "  Client: cd $(DEST) && ./sirfz --addr 127.0.0.1:9000"

transport:
	cd $(TRANSPORT_DIR) && go mod tidy && \
	go build -buildmode=c-shared -o libsirfz.so ./clib/

hypervisor:
	cd $(HYPERVISOR_DIR) && cargo build --release

run-server: all
	cd $(DEST) && ./sirfz --server --addr 0.0.0.0:9000 --lib ./libsirfz.so

run-client: all
	cd $(DEST) && ./sirfz --addr 127.0.0.1:9000 --lib ./libsirfz.so

clean:
	cd $(TRANSPORT_DIR) && rm -f libsirfz.so libsirfz.h
	cd $(HYPERVISOR_DIR) && cargo clean
