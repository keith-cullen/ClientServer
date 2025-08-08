/****************************
 *    Copyright (c) 2023    *
 *    Keith Cullen          *
 ****************************/

package main

import (
	"crypto/rand"
	"crypto/tls"
	"crypto/x509"
	"log"
	"net"
	"os"
	"time"
)

const (
	caCertPath = "../../../certs/ca.crt"
	certPath   = "../../../certs/server.crt"
	keyPath    = "../../../certs/server.key"
	network    = "tcp"
	address    = "localhost:12345"
	bufSize    = 1024
	timeout    = time.Second
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
		ClientAuth:   tls.RequireAndVerifyClientCert,
		ClientCAs:    caCertPool,
	}
	now := time.Now()
	config.Time = func() time.Time { return now }
	config.Rand = rand.Reader
	listener, err := tls.Listen(network, address, &config)
	if err != nil {
		log.Fatalf("error: %v", err)
	}
	log.Println("listening")
	var index int
	for {
		conn, err := listener.Accept()
		if err != nil {
			log.Printf("warning: %v", err)
			continue
		}
		log.Println("accepted connection from", conn.RemoteAddr())
		go func(index int) {
			log.Printf("<%v> connection open", index)
			handleConn(index, conn)
			conn.Close()
			log.Printf("<%v> connection closed", index)
		}(index)
		index++
	}
}

func handleConn(index int, conn net.Conn) {
	deadline := time.Now().Add(timeout)
	conn.SetDeadline(deadline)
	var buf [bufSize]byte
	for {
		log.Printf("<%v> receiving", index)
		n, err := conn.Read(buf[0:])
		if err != nil {
			e, ok := err.(net.Error)
			if ok && e.Timeout() {
				log.Printf("<%v> read operation timed out", index)
			} else {
				log.Printf("<%v> %v", index, err)
			}
			return
		}
		log.Printf("<%v> received: %v", index, string(buf[0:n]))

		log.Printf("<%v> sending", index)
		n, err = conn.Write(buf[0:n])
		if err != nil {
			e, ok := err.(net.Error)
			if ok && e.Timeout() {
				log.Printf("<%v> write operation timed out", index)
			} else {
				log.Printf("<%v> %v", index, err)
			}
			return
		}
		log.Printf("<%v> sent: %v", index, string(buf[0:n]))
	}
}
