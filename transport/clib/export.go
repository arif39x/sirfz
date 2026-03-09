package main

/*
#include <stdlib.h>
#include <stdint.h>
*/
import "C"
import "unsafe"

//export StartNode
func StartNode(isServer C.int, addr *C.char) C.int {
	goAddr := C.GoString(addr)
	if isServer == 1 {
		return C.int(startServer(goAddr))
	}
	return C.int(startClient(goAddr))
}

//export SendMessage
func SendMessage(handle C.int, data *C.uchar, length C.int) C.int {
	h := getHandle(int32(handle))
	if h == nil {
		return -1
	}
	buf := C.GoBytes(unsafe.Pointer(data), length)
	h.router.Broadcast(buf)
	return C.int(len(buf))
}

//export RecvMessage
func RecvMessage(handle C.int, streamIdx C.int, outBuf *C.uchar, outLen C.int) C.int {
	h := getHandle(int32(handle))
	if h == nil {
		return -1
	}
	data, err := h.router.RecvFrom(int(streamIdx))
	if err != nil {
		return -1
	}
	n := len(data)
	if n > int(outLen) {
		n = int(outLen)
	}
	dst := (*[1 << 28]C.uchar)(unsafe.Pointer(outBuf))[:n:n]
	for i, b := range data[:n] {
		dst[i] = C.uchar(b)
	}
	return C.int(n)
}

//export StopNode
func StopNode(handle C.int) {
	freeHandle(int32(handle))
}

func main() {}
