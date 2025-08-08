/****************************
 *    Copyright (c) 2023    *
 *    Keith Cullen          *
 ****************************/

package main

import (
	"log"
	"net"
	"time"
)

const (
	network = "tcp"
	address = "localhost:12345"
	bufSize = 1024
	timeout = time.Second
	numIter = 5
)

func main() {
	conn, err := net.Dial(network, address)
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
		log.Printf("sent: %v", str)

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
