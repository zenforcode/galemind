# Makefile for Go lang gRPC stub generation

PROTO_DIR ?= ../../grpc_server/proto/prediction
OUT_DIR   ?= ./galemind

PROTO_FILE = prediction.proto

.PHONY: all generate clean

setup:
	go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
	go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest

all: setup generate

build:
	go build

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
	rm -rf $(OUT_DIR)


