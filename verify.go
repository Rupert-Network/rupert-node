package main

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"fmt"
)

// Verify ...
type Verify int

// GetEncryptedMessageArgs ...
type GetEncryptedMessageArgs struct {
	account []byte
}

// GetEncryptedMessageReply ...
type GetEncryptedMessageReply struct {
	privKey rsa.PrivateKey
	message []byte
}

// GetEncryptedMessage sends the caller an encrypted message using the public key
// specified in the callers request contract ...
func (*Verify) GetEncryptedMessage(
	args *GetEncryptedMessageArgs,
	reply *GetEncryptedMessageReply,
) error {
	// TODO: get pubkey from blockchain contract call
	// TODO: cache contract calls

	// Generate keypair (will be deprecated in the future)
	privKey, err := rsa.GenerateKey(rand.Reader, 1024)
	if err != nil {
		fmt.Printf("Err generating privkey: %s", err.Error())
	}

	rng := rand.Reader
	l := []byte("Ni...")
	msg := []byte("bruh")

	// Uses public key to encrypt a secret message
	cipherText, err := rsa.EncryptOAEP(sha256.New(), rng, &privKey.PublicKey, msg, l)
	if err != nil {
		fmt.Printf("funny: %s", err)
	}

	// Returns encrypted message
	*reply = GetEncryptedMessageReply{*privKey, cipherText}

	return nil
}

// ValidateDecryptionArgs ...
type ValidateDecryptionArgs struct {
	message string
	account []byte
}

// ValidateDecryption ...
func (*Verify) ValidateDecryption(
	args *ValidateDecryptionArgs,
	reply *bool,
) error {
	*reply = true
	if args.message != "bruh" {
		*reply = false
	}

	return nil
}
