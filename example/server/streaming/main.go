package main

import (
	"flag"
	"fmt"
	"io"
	"log"
	"net"
	"strings"
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
		time.Sleep(500 * time.Millisecond)
		resp := &pb.ListFeaturesResp{
			Name: feature,
		}
		if err := stream.Send(resp); err != nil {
			return err
		}
	}
	return nil
}

// CollectFeatures is a client-side streaming RPC: it reads a stream of
// features from the client and replies with a single summary once the client
// has finished sending.
func (s *Server) CollectFeatures(stream pb.StreamingService_CollectFeaturesServer) error {
	var names []string
	for {
		req, err := stream.Recv()
		if err == io.EOF {
			return stream.SendAndClose(&pb.CollectFeaturesResp{
				Count:   int32(len(names)),
				Summary: strings.Join(names, ", "),
			})
		}
		if err != nil {
			return err
		}
		names = append(names, req.GetName())
	}
}

// EchoFeatures is a bidirectional streaming RPC: it echoes every message the
// client sends back to the client until the client closes the stream.
func (s *Server) EchoFeatures(stream pb.StreamingService_EchoFeaturesServer) error {
	for {
		req, err := stream.Recv()
		if err == io.EOF {
			return nil
		}
		if err != nil {
			return err
		}
		if err := stream.Send(&pb.EchoFeaturesResp{Name: req.GetName()}); err != nil {
			return err
		}
	}
}
