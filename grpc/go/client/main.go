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
	"time"

	app "github.com/keith-cullen/ClientServer/grpc/go/client/app"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

const (
	rootServerCert = "../../../certs/root_server_cert.pem"
	clientCert = "../../../certs/client_cert.pem"
	clientPrivkey = "../../../certs/client_privkey.pem"
	hostname = "localhost"
	port = "50051"
)

func main() {
	caCert, err := ioutil.ReadFile(rootServerCert)
	if err != nil {
		log.Fatalf("Error: %v", err)
	}
	caCertPool := x509.NewCertPool()
	caCertPool.AppendCertsFromPEM(caCert)
	cert, err := tls.LoadX509KeyPair(clientCert, clientPrivkey)
	if err != nil {
		log.Fatalf("Error: %v", err)
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
	conn, err := grpc.Dial(address, opts...)
	if err != nil {
		log.Fatalf("Error: %v", err)
	}
	defer conn.Close()
	c := app.NewAppClient(conn)
	ctx, cancel := context.WithTimeout(context.Background(), time.Second)
	defer cancel()
	name := "key1"
	r1, err := c.Get(ctx, &app.Req{Name: name})
	if err != nil {
		log.Fatalf("Error: %v", err)
	}
	log.Printf("Get Name: '%v', Value: '%v'", name, r1.Value)
}
