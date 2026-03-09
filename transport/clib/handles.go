package main

import (
	"sync"
	"sync/atomic"

	"sirfz/internal/transport"

	"github.com/hashicorp/yamux"
)

type nodeHandle struct {
	session *yamux.Session
	router  *transport.HostRouter
}

var (
	handleMu  sync.RWMutex
	handles   [64]*nodeHandle
	handleSeq int32
)

func allocHandle(h *nodeHandle) int32 {
	idx := atomic.AddInt32(&handleSeq, 1) % 64
	handleMu.Lock()
	handles[idx] = h
	handleMu.Unlock()
	return idx
}

func getHandle(idx int32) *nodeHandle {
	handleMu.RLock()
	defer handleMu.RUnlock()
	if idx < 0 || idx >= 64 {
		return nil
	}
	return handles[idx]
}

func freeHandle(idx int32) {
	handleMu.Lock()
	defer handleMu.Unlock()
	if idx >= 0 && idx < 64 {
		if h := handles[idx]; h != nil && h.session != nil {
			h.session.Close()
		}
		handles[idx] = nil
	}
}
