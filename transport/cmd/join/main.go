package main

import (
	"fmt"
	"sirfz/internal/auth"
)

func main() {
	pub, _, err := auth.GenerateEphemeralIdentity()
	if err != nil {
		fmt.Printf("identity generation failed: %v\n", err)
		return
	}
	fmt.Printf("[sirfz] ephemeral node public key: %x\n", pub)
}
