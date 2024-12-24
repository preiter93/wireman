package main

import (
	"flag"
	"fmt"
	"log"
	"net"
	"time"

	pb "grpc_simple/proto"

	"google.golang.org/grpc"
)

var (
	port = flag.Int("port", 50051, "The server port")
)

type Server struct {
	pb.UnimplementedStreamingServiceServer
}

func main() {
	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", *port))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	s := grpc.NewServer()
	pb.RegisterStreamingServiceServer(s, &Server{})

	fmt.Println("Listening on", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Failed to serve grpc Server: %v", err)
	}
}

func (s *Server) ListFeatures(req *pb.ListFeaturesReq, stream pb.StreamingService_ListFeaturesServer) error {
	features := []string{
		"Upload Files",
		"Delete Files",
		"1TB Storage",
		"Share Files with Others",
		"Automated Backup",
		"Advanced Search",
		"Bulk File Upload",
		"File Preview",
	}

	for _, feature := range features {
		time.Sleep(1 * time.Second)
		resp := &pb.ListFeaturesResp{
			Name: feature,
		}
		if err := stream.Send(resp); err != nil {
			return err
		}
	}
	return nil
}
