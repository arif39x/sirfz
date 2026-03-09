package transport

import (
	"io"
	"sync"
	"sync/atomic"

	"github.com/hashicorp/yamux"
)

const maxPeers = 64
const relayBufSize = 32 * 1024

type HostRouter struct {
	mu      sync.RWMutex
	streams [maxPeers]*yamux.Stream
	count   int32
}

var relayPool = sync.Pool{
	New: func() any {
		b := make([]byte, relayBufSize)
		return &b
	},
}

func NewHostRouter() *HostRouter {
	return &HostRouter{}
}

func (r *HostRouter) Register(s *yamux.Stream) int {
	r.mu.Lock()
	defer r.mu.Unlock()

	for i, slot := range r.streams {
		if slot == nil {
			r.streams[i] = s
			atomic.AddInt32(&r.count, 1)
			return i
		}
	}
	return -1
}

func (r *HostRouter) Deregister(idx int) {
	r.mu.Lock()
	defer r.mu.Unlock()

	if idx >= 0 && idx < maxPeers && r.streams[idx] != nil {
		r.streams[idx] = nil
		atomic.AddInt32(&r.count, -1)
	}
}

func (r *HostRouter) Broadcast(ciphertext []byte) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	for _, s := range r.streams {
		if s == nil {
			continue
		}
		buf := relayPool.Get().(*[]byte)
		n := copy(*buf, ciphertext)
		_, _ = s.Write((*buf)[:n])
		zeroizeRelay(*buf)
		relayPool.Put(buf)
	}
}

func (r *HostRouter) RecvFrom(idx int) ([]byte, error) {
	r.mu.RLock()
	s := r.streams[idx]
	r.mu.RUnlock()

	if s == nil {
		return nil, io.EOF
	}

	buf := make([]byte, relayBufSize)
	n, err := s.Read(buf)
	if err != nil {
		return nil, err
	}
	result := make([]byte, n)
	copy(result, buf[:n])
	zeroizeRelay(buf)
	return result, nil
}

func zeroizeRelay(b []byte) {
	for i := range b {
		b[i] = 0
	}
}
