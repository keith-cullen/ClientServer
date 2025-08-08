/****************************
 *    Copyright (c) 2023    *
 *    Keith Cullen          *
 ****************************/

package main

import (
	"crypto/tls"
	"crypto/x509"
	"log"
	"net"
	"os"
	"time"
)

const (
	caCertPath = "../../../certs/ca.crt"
	certPath   = "../../../certs/client.crt"
	keyPath    = "../../../certs/client.key"
	network    = "tcp"
	address    = "localhost:12345"
	bufSize    = 1024
	timeout    = time.Second
	numIter    = 5
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
	config := tls.Config{
		Certificates: []tls.Certificate{cert},
		RootCAs:      caCertPool,
	}
	conn, err := tls.Dial(network, address, &config)
	if err != nil {
		log.Fatalf("error: %v", err)
	}
	log.Printf("opened connection to %s", conn.RemoteAddr())
	handleConn(conn)
	conn.Close()
	log.Printf("connection closed")
}

func handleConn(conn net.Conn) {
	deadline := time.Now().Add(timeout)
	conn.SetDeadline(deadline)
	var buf [bufSize]byte
	for n := 0; n < numIter; n++ {
		log.Println("sending")
		str := "hello" + string(n+48)
		n, err := conn.Write([]byte(str))
		if err != nil {
			e, ok := err.(net.Error)
			if ok && e.Timeout() {
				log.Fatalf("write operation timed out")
			}
			log.Fatalf("error: %v", err)
		}
		log.Printf("Sent: %v", str)

		log.Println("receiving")
		n, err = conn.Read(buf[0:])
		if err != nil {
			e, ok := err.(net.Error)
			if ok && e.Timeout() {
				log.Fatalf("read operation timed out")
			}
			log.Fatalf("error: %v", err)
		}
		log.Printf("received: %v", string(buf[0:n]))
	}
}
