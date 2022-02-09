package main

import (
	"encoding/base64"
	"testing"
)

func TestVerify(t *testing.T) {
	// Create new Verify
	v := new(Verify)
	// Empty account until read from chain implemented
	args := struct{ account []byte }{[]byte(":)")}
	// Byte array to store response
	var reply []byte

	// Call
	v.GetEncryptedMessage(
		&args,
		&reply,
	)

	// As GetEncryptedMessage is incomplete this is just to demonstrate the
	// passing of the message back
	if string(reply) != ":(" {
		t.Fatalf("Crazy it was actually %s",
			base64.StdEncoding.EncodeToString(reply),
		)
	}
}
