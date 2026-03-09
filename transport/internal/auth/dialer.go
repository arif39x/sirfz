package auth

import (
	"crypto/tls"
	"fmt"
	"net"

	"github.com/hashicorp/yamux"
)

func DialEphemeral(addr string, tlsCfg *tls.Config) (*yamux.Session, error) {
	rawConn, err := net.Dial("tcp", addr)
	if err != nil {
		return nil, fmt.Errorf("tcp dial failed: %w", err)
	}

	tlsConn := tls.Client(rawConn, tlsCfg)
	if err := tlsConn.Handshake(); err != nil {
		rawConn.Close()
		return nil, fmt.Errorf("tls handshake failed: %w", err)
	}

	yCfg := yamux.DefaultConfig()
	yCfg.EnableKeepAlive = true

	session, err := yamux.Client(tlsConn, yCfg)
	if err != nil {
		tlsConn.Close()
		return nil, fmt.Errorf("yamux client failed: %w", err)
	}

	return session, nil
}

func ListenEphemeral(addr string, tlsCfg *tls.Config) (*yamux.Session, error) {
	ln, err := tls.Listen("tcp", addr, tlsCfg)
	if err != nil {
		return nil, fmt.Errorf("tls listen failed: %w", err)
	}
	defer ln.Close()

	conn, err := ln.Accept()
	if err != nil {
		return nil, fmt.Errorf("accept failed: %w", err)
	}

	yCfg := yamux.DefaultConfig()
	yCfg.EnableKeepAlive = true

	session, err := yamux.Server(conn, yCfg)
	if err != nil {
		conn.Close()
		return nil, fmt.Errorf("yamux server setup failed: %w", err)
	}

	return session, nil
}
