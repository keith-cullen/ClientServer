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
	address = "0.0.0.0:12345"
	bufSize = 1024
	timeout = time.Second
)

func main() {
	listener, err := net.Listen(network, address)
	if err != nil {
		log.Fatalf("Error: %v", err)
	}
	log.Println("Listening")
	var index int
	for {
		conn, err := listener.Accept()
		if err != nil {
			log.Printf("Warning: %v", err)
			continue
		}
		log.Println("Accepted connection from", conn.RemoteAddr())
		go handleConn(index, conn)
		index++
	}
}

func handleConn(index int, conn net.Conn) {
	defer conn.Close()
	deadline := time.Now().Add(timeout)
	conn.SetDeadline(deadline)
	log.Printf("<%v> Connection open", index)
	defer log.Printf("<%v> Connection closed", index)
	var buf [bufSize]byte
	for {
		log.Printf("<%v> Receiving", index)
		n, err := conn.Read(buf[0:])
		if err != nil {
			e, ok := err.(net.Error)
			if ok && e.Timeout() {
				log.Printf("<%v> Read operation timed out", index)
			} else {
				log.Printf("<%v> %v", index, err)
			}
			return
		}
		log.Printf("<%v> Received: %v", index, string(buf[0:n]))

		log.Printf("<%v> Sending", index)
		n, err = conn.Write(buf[0:n])
		if err != nil {
			e, ok := err.(net.Error)
			if ok && e.Timeout() {
				log.Printf("<%v> Write operation timed out", index)
			} else {
				log.Printf("<%v> %v", index, err)
			}
			return
		}
		log.Printf("<%v> Sent: %v", index, string(buf[0:n]))
	}
}
