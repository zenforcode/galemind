# Galemind Protobuf Clients

This directory contains Protocol Buffers (Protobuf) client implementations for interacting with Galemind services in **Go**, **Python**, and **Java**.

---

## 📦 Supported Languages

- [Go](#go-client)
- [Python](#python-client)
- [Java](#java-client)

---

## 📄 Protobuf Specification

All client libraries are generated from the shared Protobuf definitions located in the `proto/` directory.

To regenerate client code after modifying `.proto` files, follow the language-specific instructions below.

---

## 🔧 Prerequisites

- [Protocol Buffers Compiler (`protoc`)](https://grpc.io/docs/protoc-installation/)
- Language-specific plugins (see below)

---

## 🚀 Go Client

### 📂 Location
`/src/clients/go/`

### 📦 Requirements

- Go 1.18+
- `protoc-gen-go` and `protoc-gen-go-grpc`

### 🛠️ Installation

```bash
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
```
## Python Client
📂 Location
/src/clients/python/

### Requirements
Python 3.8+

grpcio, grpcio-tools

### Installation
```
pip install grpcio grpcio-tools
```