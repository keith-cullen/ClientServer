/****************************
 *    Copyright (c) 2023    *
 *    Keith Cullen          *
 ****************************/

package main

import (
	"context"
	"crypto/tls"
	"crypto/x509"
	"log"
	"os"
	"time"

	app "github.com/keith-cullen/ClientServer/grpc/go/client/app"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

const (
	caCertPath = "../../../certs/ca.crt"
	certPath   = "../../../certs/client.crt"
	keyPath    = "../../../certs/client.key"
	hostname   = "localhost"
	port       = "50051"
)

func main() {
	caCert, err := os.ReadFile(caCertPath)
	if err != nil {
		log.Fatalf("error: %v", err)
	}
	caCertPool := x509.NewCertPool()
	caCertPool.AppendCertsFromPEM(caCert)
	cert, err := tls.LoadX509KeyPair(certPath, keyPath)
	if err != nil {
		log.Fatalf("error: %v", err)
	}
	tlsConfig := tls.Config{
		ServerName:   hostname,
		Certificates: []tls.Certificate{cert},
		RootCAs:      caCertPool,
	}
	opts := []grpc.DialOption{
		grpc.WithTransportCredentials(credentials.NewTLS(&tlsConfig)),
	}
	address := hostname + ":" + port
	conn, err := grpc.NewClient(address, opts...)
	if err != nil {
		log.Fatalf("error: %v", err)
	}
	defer conn.Close()
	c := app.NewAppClient(conn)
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	name := "key1"
	r1, err := c.Get(ctx, &app.Req{Name: name})
	if err != nil {
		log.Fatalf("error: %v", err)
	}
	log.Printf("get name: '%v', value: '%v'", name, r1.Value)
}
