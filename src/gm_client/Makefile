# Makefile for Python gRPC stub generation

# You can override these on the command line:
# make PROTO_DIR=path/to/protos OUT_DIR=path/to/output
PROTO_DIR ?= ../grpc_server/proto/prediction
OUT_DIR   ?= ./galemind

# Proto file name (without path)
PROTO_FILE = prediction.proto

.PHONY: all generate clean

all: generate

## Generate Python gRPC code from .proto
generate:
	@echo "Generating Python gRPC stubs from $$PROTO_DIR/$$PROTO_FILE → $$OUT_DIR"
	mkdir -p $(OUT_DIR)
	python -m grpc_tools.protoc \
		--proto_path=$(PROTO_DIR) \
		--python_out=$(OUT_DIR) \
		--grpc_python_out=$(OUT_DIR) \
		$(PROTO_DIR)/$(PROTO_FILE)
	touch $(OUT_DIR)/__init__.py

## Remove generated files
clean:
	@echo "Cleaning generated files in $(OUT_DIR)"
	rm -rf $(OUT_DIR)/*


