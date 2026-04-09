package auth

import (
	"crypto/hmac"
	"crypto/rand"
	"crypto/sha256"
	"errors"
	"io"
	"net"
	"time"
)

func SecureHandshakeClient(conn net.Conn, authKey []byte) error {
	nonce := make([]byte, 32)
	if _, err := io.ReadFull(rand.Reader, nonce); err != nil {
		return err
	}

	mac := hmac.New(sha256.New, authKey)
	mac.Write(nonce)
	signature := mac.Sum(nil)

	payload := append(nonce, signature...)

	conn.SetWriteDeadline(time.Now().Add(5 * time.Second))
	if _, err := conn.Write(payload); err != nil {
		return err
	}
	conn.SetWriteDeadline(time.Time{})

	return nil
}

func SecureHandshakeServer(conn net.Conn, authKey []byte) error {
	payload := make([]byte, 64)

	conn.SetReadDeadline(time.Now().Add(5 * time.Second))
	if _, err := io.ReadFull(conn, payload); err != nil {
		return err
	}
	conn.SetReadDeadline(time.Time{})

	nonce := payload[:32]
	signature := payload[32:]

	mac := hmac.New(sha256.New, authKey)
	mac.Write(nonce)
	expectedSignature := mac.Sum(nil)

	if !hmac.Equal(signature, expectedSignature) {
		return errors.New("invalid signature")
	}

	return nil
}
