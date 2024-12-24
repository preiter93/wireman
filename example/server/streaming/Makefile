GOCMD=go
GOBUILD=$(GOCMD) build
GOCLEAN=$(GOCMD) clean
PROTOC=protoc
PROTOS=

PROTOC_GEN_GO=--go_out=./proto --go_opt=paths=source_relative --go-grpc_out=./proto --go-grpc_opt=paths=source_relative

BINARY_NAME=streaming

out = "."

all: clean build run

build:
	$(PROTOC) $(PROTOC_GEN_GO) streaming.proto
	$(GOBUILD) -o $(BINARY_NAME) -v ./main.go

clean:
	$(GOCLEAN)
	rm -f $(BINARY_NAME)
	rm -f proto/streaming.pb.go
	rm -f proto/streaming_grpc.pb.go

run: build
	./$(BINARY_NAME)
