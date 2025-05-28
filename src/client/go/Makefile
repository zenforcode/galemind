# Makefile for Python gRPC stub generation

# You can override these on the command line:
# make PROTO_DIR=path/to/protos OUT_DIR=path/to/output
PROTO_DIR ?= ../../grpc_server/proto/prediction
OUT_DIR   ?= ./galemind

# Proto file name (without path)
PROTO_FILE = prediction.proto

.PHONY: all generate clean

setup:
	go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
	go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

all: setup generate

## Generate Python gRPC code from .proto
generate:
	@echo "Generating Go Lang gRPC stubs from $$PROTO_DIR/$$PROTO_FILE → $$OUT_DIR"
	mkdir -p $(OUT_DIR)
	protoc \
 	--go_out=$(OUT_DIR) \
  	--go-grpc_out=$(OUT_DIR) \
  	--proto_path=$(PROTO_DIR) \
	$(PROTO_DIR)/$(PROTO_FILE)

## Remove generated files
clean:
	@echo "Cleaning generated files in $(OUT_DIR)"
	rm -rf $(OUT_DIR)/*


