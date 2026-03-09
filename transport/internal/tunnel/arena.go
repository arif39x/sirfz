package tunnel

import (
	"sync"
)

const ArenaBufferSize = 32 * 1024

var bufferArena = sync.Pool{
	New: func() any {
		b := make([]byte, ArenaBufferSize)
		return &b
	},
}

func acquireBuffer() *[]byte {
	return bufferArena.Get().(*[]byte)
}

func releaseBuffer(b *[]byte) {
	zeroize(*b)
	bufferArena.Put(b)
}

func zeroize(b []byte) {
	for i := range b {
		b[i] = 0
	}
}
