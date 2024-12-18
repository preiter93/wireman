GOCMD=go
GOBUILD=$(GOCMD) build
GOCLEAN=$(GOCMD) clean
PROTOC=protoc
PROTOS=

PROTOC_GEN_GO=--go_out=./proto --go_opt=paths=source_relative --go-grpc_out=./proto --go-grpc_opt=paths=source_relative

BINARY_NAME=main

out = "."

all: clean build run

build:
	$(PROTOC) $(PROTOC_GEN_GO) order/api.proto price/api.proto
	$(GOBUILD) -o $(BINARY_NAME) -v ./main.go

clean:
	$(GOCLEAN)
	rm -f $(BINARY_NAME)
	rm -f proto/order/api.pb.go
	rm -f proto/order/api_grpc.pb.go
	rm -f proto/price/api.pb.go
	rm -f proto/price/api_grpc.pb.go

run: build
	./$(BINARY_NAME)
