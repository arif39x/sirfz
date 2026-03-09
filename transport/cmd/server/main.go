package main

import (
	"fmt"
	"net"
	"sirfz/internal/transport"
	"sirfz/internal/tunnel"

	"github.com/hashicorp/yamux"
)

func main() {
	yCfg := yamux.DefaultConfig()
	yCfg.EnableKeepAlive = true

	ln, err := net.Listen("tcp", ":7000")
	if err != nil {
		fmt.Printf("listen failed: %v\n", err)
		return
	}
	defer ln.Close()

	fmt.Println("[sirfz] relay listening on :7000")

	router := transport.NewHostRouter()

	for {
		conn, err := ln.Accept()
		if err != nil {
			return
		}
		session, err := tunnel.SetupSession(conn, true, tunnel.DefaultConfig)
		if err != nil {
			conn.Close()
			continue
		}
		go func(sess *yamux.Session) {
			for {
				stream, err := sess.AcceptStream()
				if err != nil {
					return
				}
				idx := router.Register(stream)
				if idx < 0 {
					stream.Close()
				}
			}
		}(session)
	}
}
