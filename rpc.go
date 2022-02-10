package main

import (
	"net/rpc"
)

// RegisterRPC ...
func RegisterRPC() {
	rpc.Register(new(Verify))
}
