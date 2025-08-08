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
	timeout = time.Second
	bufSize = 1024
)

func main() {
	listener, err := net.Listen(network, address)
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
