/****************************
 *    Copyright (c) 2023    *
 *    Keith Cullen          *
 ****************************/

package main

import (
	"context"
	"crypto/tls"
	"crypto/x509"
	"io/ioutil"
	"log"
	"net"

	app "github.com/keith-cullen/ClientServer/grpc/go/server/app"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/status"
)

const (
	rootClientCert = "../../../certs/root_client_cert.pem"
	serverCert = "../../../certs/server_cert.pem"
	serverPrivkey = "../../../certs/server_privkey.pem"
	port = ":50051"
)

type server struct {
	app.UnimplementedAppServer
}

func main() {
	caCert, err := ioutil.ReadFile(rootClientCert)
	if err != nil {
		log.Fatalf("Error: %v", err)
	}
	caCertPool := x509.NewCertPool()
	caCertPool.AppendCertsFromPEM(caCert)
	cert, err := tls.LoadX509KeyPair(serverCert, serverPrivkey)
	if err != nil {
		log.Fatalf("Error: %v", err)
	}
	tlsConfig := tls.Config{
		Certificates: []tls.Certificate{cert},
		ClientAuth:   tls.RequireAndVerifyClientCert,
		ClientCAs:    caCertPool,
	}
	opts := []grpc.ServerOption{
		grpc.Creds(credentials.NewTLS(&tlsConfig)),
	}
	s := grpc.NewServer(opts...)
	app.RegisterAppServer(s, &server{})
	lis, err := net.Listen("tcp", port)
	if err != nil {
		log.Fatalf("Error: %v", err)
	}
	log.Printf("Starting gRPC listener")
	if err := s.Serve(lis); err != nil {
		log.Fatalf("Error: %v", err)
	}
}

func (s *server) Get(ctx context.Context, req *app.Req) (*app.Resp, error) {
	if req.Name == "key1" {
		val := "val1"
		log.Printf("Get Name: '%v', Value: '%v'", req.Name, val)
		return &app.Resp{Value: val}, status.New(codes.OK, "").Err()
	}
	return nil, status.Errorf(codes.NotFound, "Name not found '%v'", req.Name)
}
