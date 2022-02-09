package main

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"fmt"
	"net/rpc"
)

// Arith ...
type Arith int

// Multiply ...
func (*Arith) Multiply(
	args *struct{ A, B int },
	reply *int,
) error {
	*reply = args.A * args.B
	return nil
}

type Verify int

func (*Verify) GetEncryptedMessage(
	args *struct{ account []byte },
	reply *[]byte,
) error {
	// TODO: get pubkey from blockchain contract call
	// TODO: cache contract calls

	// Generate keypair (will be deprecated in the future)
	privKey, err := rsa.GenerateKey(rand.Reader, 1024)
	if err != nil {
		fmt.Printf("Err generating privkey: %s", err.Error())
	}
	pubkey := privKey.PublicKey

	rng := rand.Reader
	label := []byte("Ni...")
	message := []byte("bruh")

	// Uses public key to encrypt a secret message
	ciphertext, err := rsa.EncryptOAEP(
		sha256.New(),
		rng,
		&pubkey,
		message,
		label,
	)

	if err != nil {
		fmt.Printf("funny: %s", err)
	}

	// Returns encrypted message
	*reply = ciphertext

	return nil
}

func (*Verify) ValidateDecryption(
	args struct{ message string },
	reply *bool,
) error {
	check := args.message == "bruh"
	reply = &check

	return nil
}

// RegisterRPC ...
func RegisterRPC() {
	rpc.Register(new(Arith))
}
