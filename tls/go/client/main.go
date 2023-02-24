/****************************
 *    Copyright (c) 2023    *
 *    Keith Cullen          *
 ****************************/

package main

import (
	"crypto/tls"
	"crypto/x509"
	"io/ioutil"
	"log"
	"net"
	"time"
)

const (
	clientCert = "../../../certs/client_cert.pem"
	clientPrivkey = "../../../certs/client_privkey.pem"
	rootServerCert = "../../../certs/root_server_cert.pem"
	network = "tcp"
	address = "localhost:12345"
	bufSize = 1024
	timeout = time.Second
	numIter = 5
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
	config := tls.Config{
		Certificates: []tls.Certificate{cert},
		RootCAs: caCertPool,
	}
	conn, err := tls.Dial(network, address, &config)
	if err != nil {
		log.Fatalf("Error: %v", err)
	}
	log.Printf("Opened connection to %s", conn.RemoteAddr())
	handleConn(conn)
}

func handleConn(conn net.Conn) {
	defer conn.Close()
	deadline := time.Now().Add(timeout)
	conn.SetDeadline(deadline)
	defer log.Printf("Connection closed");
	var buf [bufSize]byte
	for n := 0; n < numIter; n++ {
		log.Println("Sending")
		str := "hello" + string(n + 48)
		n, err := conn.Write([]byte(str))
		if err != nil {
			e, ok := err.(net.Error)
			if ok && e.Timeout() {
				log.Fatalf("Write operation timed out")
			}
			log.Fatalf("Error: %v", err)
		}
		log.Printf("Sent: %v", str)

		log.Println("Receiving")
		n, err = conn.Read(buf[0:])
		if err != nil {
			e, ok := err.(net.Error)
			if ok && e.Timeout() {
				log.Fatalf("Read operation timed out")
			}
			log.Fatalf("Error: %v", err)
		}
		log.Printf("Received: %v", string(buf[0:n]))
	}
}

