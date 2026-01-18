package main

import (
	"context"
	"fmt"
	"log"
	"net"
	"runtime"
	"time"

	pb "axon-agent/proto"
	"google.golang.org/grpc"
	"google.golang.org/grpc/reflection"
)

type server struct {
	pb.UnimplementedSystemControlServer
}

func (s *server) GetStats(ctx context.Context, in *pb.Empty) (*pb.SystemStats, error) {
	var m runtime.MemStats
	runtime.ReadMemStats(&m)

	return &pb.SystemStats{
		CpuUsage: 12.5,
		RamUsage: float32(m.Alloc) / 1024 / 1024,
		Uptime:   float32(time.Since(startTime).Seconds()),
	}, nil
}

var startTime time.Time

func main() {
	startTime = time.Now()

	// Start Heartbeat in background (Hub listens on 50052)
	go startHeartbeat("127.0.0.1:50052")

	port := 50051
	lis, err := net.Listen("tcp", fmt.Sprintf("127.0.0.1:%d", port))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	s := grpc.NewServer()
	pb.RegisterSystemControlServer(s, &server{})
	reflection.Register(s)

	fmt.Printf("Agent listening on port %d\n", port)
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
