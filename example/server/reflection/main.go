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
	"google.golang.org/grpc/reflection"
)

var (
	port = flag.Int("port", 50051, "The server port")
)

type Server struct {
	pb.UnimplementedOrderServiceServer
	pb.UnimplementedDebuggerServer
}

func main() {
	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", *port))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	s := grpc.NewServer()
	pb.RegisterOrderServiceServer(s, &Server{})
	pb.RegisterDebuggerServer(s, &Server{})

	reflection.Register(s)

	fmt.Println("Listening on", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Failed to serve grpc Server: %v", err)
	}
}

func (s *Server) Metadata(ctx context.Context, req *pb.MetadataReq) (*pb.MetadataResp, error) {
	md, ok := metadata.FromIncomingContext(ctx)
	if !ok {
		return nil, fmt.Errorf("failed to get metadata from context")
	}

	mdJson, err := json.Marshal(md)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal metadata to JSON: %v", err)
	}

	return &pb.MetadataResp{
		Metadata: string(mdJson),
	}, nil
}

func (s *Server) GetOrder(ctx context.Context, req *pb.GetOrderReq) (*pb.GetOrderResp, error) {
	order := &pb.Order{
		Id:   req.OrderId,
		Name: "Dummy Order",
	}
	return &pb.GetOrderResp{
		Order: order,
	}, nil
}

func (s *Server) ListOrders(ctx context.Context, req *pb.ListOrdersReq) (*pb.ListOrdersResp, error) {
	order := &pb.Order{
		Id:   "1",
		Name: "Dummy Order",
	}
	return &pb.ListOrdersResp{
		Orders: []*pb.Order{order},
	}, nil
}
