GOCMD=go
GOBUILD=$(GOCMD) build
GOCLEAN=$(GOCMD) clean
PROTOC=protoc
PROTOS=

PROTOC_GEN_GO=--go_out=./proto --go_opt=paths=source_relative --go-grpc_out=./proto --go-grpc_opt=paths=source_relative

BINARY_NAME=reflection

out = "."

all: clean build run

build:
	$(PROTOC) $(PROTOC_GEN_GO) order.proto debugger.proto
	$(GOBUILD) -o $(BINARY_NAME) -v ./main.go

clean:
	$(GOCLEAN)
	rm -f $(BINARY_NAME)
	rm -f proto/order.pb.go
	rm -f proto/order_grpc.pb.go
	rm -f proto/debugger.pb.go
	rm -f proto/debugger_grpc.pb.go

run: build
	./$(BINARY_NAME)
