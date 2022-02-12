package main

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"testing"
)

func TestVerify(t *testing.T) {
	// Create new Verify
	v := new(Verify)
	// Empty account until read from chain implemented
	args := GetEncryptedMessageArgs{[]byte(":)")}
	// Byte array to store response
	var reply GetEncryptedMessageReply

	// Call
	if err := v.GetEncryptedMessage(&args, &reply); err != nil {
		t.Fatalf("error getting encrypted message: %s", err)
	}

	label := []byte("Ni...")
	rng := rand.Reader
	plaintext, err := rsa.DecryptOAEP(sha256.New(), rng, &reply.privKey, reply.message, label)
	if err != nil {
		t.Fatalf("error decrypting: %s", err)
	}

	vArgs := ValidateDecryptionArgs{
		string(plaintext),
		[]byte(":)"),
	}
	var vReply bool

	if err = v.ValidateDecryption(&vArgs, &vReply); err != nil {
		t.Fatalf("error validating decryption: %s", err)
	}
}
