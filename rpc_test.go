package main

import (
	"encoding/base64"
	"testing"
)

func TestVerify(t *testing.T) {
	v := new(Verify)
	args := struct{ account []byte }{[]byte(":)")}
	var reply []byte

	v.GetEncryptedMessage(
		&args,
		&reply,
	)

	if string(reply) != ":(" {
		t.Fatalf("Crazy it was actually %s",
			base64.StdEncoding.EncodeToString(reply),
		)
	}
}
