package main

import (
	"context"
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"net"

	pb "grpc_simple/proto"

	"google.golang.org/grpc"
	"google.golang.org/grpc/metadata"
	timestamppb "google.golang.org/protobuf/types/known/timestamppb"
)

var (
	port = flag.Int("port", 50051, "The server port")
)

type Server struct {
	pb.UnimplementedGreeterServer
	pb.UnimplementedTimeKeeperServer
	pb.UnimplementedDebuggerServer
}

func (s *Server) SayHello(ctx context.Context, req *pb.HelloReq) (*pb.HelloResp, error) {
	return &pb.HelloResp{Message: fmt.Sprintf("Hello %v", req.GetName())}, nil
}

func (s *Server) GetDate(ctx context.Context, req *pb.GetDateReq) (*pb.GetDateResp, error) {
	now := timestamppb.Now()
	return &pb.GetDateResp{Date: now}, nil
}

func (s *Server) GetNameOfMonth(ctx context.Context, req *pb.GetNameOfMonthReq) (*pb.GetNameOfMonthResp, error) {
	switch req.Number {
	case 1:
		return &pb.GetNameOfMonthResp{Name: "January"}, nil
	case 2:
		return &pb.GetNameOfMonthResp{Name: "February"}, nil
	case 3:
		return &pb.GetNameOfMonthResp{Name: "March"}, nil
	case 4:
		return &pb.GetNameOfMonthResp{Name: "April"}, nil
	case 5:
		return &pb.GetNameOfMonthResp{Name: "May"}, nil
	case 6:
		return &pb.GetNameOfMonthResp{Name: "June"}, nil
	case 7:
		return &pb.GetNameOfMonthResp{Name: "July"}, nil
	case 8:
		return &pb.GetNameOfMonthResp{Name: "August"}, nil
	case 9:
		return &pb.GetNameOfMonthResp{Name: "September"}, nil
	case 10:
		return &pb.GetNameOfMonthResp{Name: "October"}, nil
	case 11:
		return &pb.GetNameOfMonthResp{Name: "November"}, nil
	case 12:
		return &pb.GetNameOfMonthResp{Name: "December"}, nil
	default:
		return nil, fmt.Errorf("number must be 1..12, got %v", req.Number)
	}
}

func (s *Server) GetMetadata(ctx context.Context, req *pb.GetMetadataReq) (*pb.GetMetadataResp, error) {
	// Get metadata from the incoming context
	md, ok := metadata.FromIncomingContext(ctx)
	if !ok {
		return nil, fmt.Errorf("failed to get metadata from context")
	}

	// Convert metadata map to JSON string
	mdJson, err := json.Marshal(md)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal metadata to JSON: %v", err)
	}

	// Create the response message
	resp := &pb.GetMetadataResp{
		Metadata: string(mdJson),
	}

	return resp, nil
}

func main() {
	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", *port))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	s := grpc.NewServer()
	pb.RegisterGreeterServer(s, &Server{})
	pb.RegisterTimeKeeperServer(s, &Server{})
	pb.RegisterDebuggerServer(s, &Server{})

	fmt.Println("Listening on", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Failed to serve grpc Server: %v", err)
	}
}
