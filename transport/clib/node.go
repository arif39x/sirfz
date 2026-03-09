package main

import (
	"net"

	"sirfz/internal/transport"

	"github.com/hashicorp/yamux"
)

func startServer(addr string) int32 {
	router := transport.NewHostRouter()

	yCfg := yamux.DefaultConfig()
	yCfg.EnableKeepAlive = true

	ln, err := net.Listen("tcp", addr)
	if err != nil {
		return -1
	}

	go func() {
		for {
			conn, err := ln.Accept()
			if err != nil {
				return
			}
			session, err := yamux.Server(conn, yCfg)
			if err != nil {
				conn.Close()
				continue
			}
			go acceptStreams(session, router)
		}
	}()

	return allocHandle(&nodeHandle{router: router})
}

func acceptStreams(sess *yamux.Session, router *transport.HostRouter) {
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
}

func startClient(addr string) int32 {
	yCfg := yamux.DefaultConfig()
	yCfg.EnableKeepAlive = true

	conn, err := net.Dial("tcp", addr)
	if err != nil {
		return -1
	}

	session, err := yamux.Client(conn, yCfg)
	if err != nil {
		conn.Close()
		return -1
	}

	stream, err := session.OpenStream()
	if err != nil {
		session.Close()
		return -1
	}

	router := transport.NewHostRouter()
	router.Register(stream)

	return allocHandle(&nodeHandle{session: session, router: router})
}
