package main

import (
	"context"
	"log"
	"os"
	"time"

	pb "axon-agent/proto"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

func startHeartbeat(hubAddr string) {
	conn, err := grpc.NewClient(hubAddr, grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Printf("Failed to connect to Hub: %v", err)
		return
	}
	defer conn.Close()

	client := pb.NewLifeSupportClient(conn)
	missedPulses := 0

	for {
		ctx, cancel := context.WithTimeout(context.Background(), time.Second)
		_, err := client.Heartbeat(ctx, &pb.Pulse{
			Timestamp: time.Now().Unix(),
			Status:    "ALIVE",
		})
		cancel()

		if err != nil {
			missedPulses++
			log.Printf("Missed heartbeat: %v (%d/4)", err, missedPulses)
			if missedPulses >= 4 {
				log.Fatalf("Lost connection to Hub. Self-destructing.")
				os.Exit(1) // Dead Man's Switch Triggered
			}
		} else {
			missedPulses = 0
		}

		time.Sleep(500 * time.Millisecond)
	}
}
