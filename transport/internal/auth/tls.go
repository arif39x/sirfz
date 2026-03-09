package auth

import (
	"crypto/tls"
	"crypto/x509"
)

func BuildTLSConfig(certDER []byte, privDER []byte, caDER []byte, isServer bool) (*tls.Config, error) {
	cert, err := tls.X509KeyPair(certDER, privDER)
	if err != nil {
		return nil, err
	}

	caPool := x509.NewCertPool()
	caPool.AppendCertsFromPEM(caDER)

	cfg := &tls.Config{
		Certificates: []tls.Certificate{cert},
		RootCAs:      caPool,
		MinVersion:   tls.VersionTLS13,
	}

	if isServer {
		cfg.ClientCAs = caPool
		cfg.ClientAuth = tls.RequireAndVerifyClientCert
	}

	return cfg, nil
}
