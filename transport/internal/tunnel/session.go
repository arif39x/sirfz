package tunnel

import (
	"io"
	"log"
	"net"
	"time"

	"github.com/hashicorp/yamux"
)

type Config struct {
	YamuxMaxWindow int
}

var DefaultConfig = Config{
	YamuxMaxWindow: 256 * 1024,
}

func SetupSession(conn net.Conn, isServer bool, cfg Config) (*yamux.Session, error) {
	yCfg := yamux.DefaultConfig()
	yCfg.MaxStreamWindowSize = uint32(cfg.YamuxMaxWindow)
	yCfg.EnableKeepAlive = true
	yCfg.Logger = log.New(io.Discard, "", 0)

	if isServer {
		return yamux.Server(conn, yCfg)
	}
	return yamux.Client(conn, yCfg)
}

func Relay(a, b net.Conn, timeout time.Duration) {
	defer a.Close()
	defer b.Close()

	done := make(chan struct{}, 2)

	go func() { copyStream(a, b); done <- struct{}{} }()
	go func() { copyStream(b, a); done <- struct{}{} }()

	<-done
}

func copyStream(dst io.Writer, src io.Reader) {
	buf := acquireBuffer()
	defer releaseBuffer(buf)

	for {
		n, readErr := src.Read(*buf)
		if n > 0 {
			written := 0
			for written < n {
				nw, writeErr := dst.Write((*buf)[written:n])
				written += nw
				if writeErr != nil {
					return
				}
			}
		}
		if readErr != nil {
			return
		}
	}
}
