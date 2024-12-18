package main

import (
	"context"
	"flag"
	"fmt"
	"log"
	"net"
	"time"

	pbOrder "grpc_website/proto/order"
	pbPrice "grpc_website/proto/price"

	"google.golang.org/grpc"
	"google.golang.org/protobuf/types/known/timestamppb"
)

var (
	port = flag.Int("port", 50051, "The server port")
)

type Server struct {
	pbOrder.UnimplementedOrderServiceServer
	pbPrice.UnimplementedPriceServiceServer
}

func (s *Server) GetOrder(ctx context.Context, req *pbOrder.GetOrderReq) (*pbOrder.GetOrderResp, error) {
	return &pbOrder.GetOrderResp{
		Order: &pbOrder.Order{
			Id:        "123",
			Name:      "Sample Order",
			Status:    "shipped",
			CreatedAt: timestamppb.New(time.Date(2024, 12, 1, 10, 0, 0, 0, time.UTC)),
			UpdatedAt: timestamppb.New(time.Now()),
		},
	}, nil
}

func main() {
	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", *port))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	s := grpc.NewServer()
	pbOrder.RegisterOrderServiceServer(s, &Server{})
	pbPrice.RegisterPriceServiceServer(s, &Server{})

	fmt.Println("Listening on", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Failed to serve grpc Server: %v", err)
	}
}
