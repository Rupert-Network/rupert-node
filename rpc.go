package main

import (
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

// RegisterRPC ...
func RegisterRPC() {
	rpc.Register(new(Arith))
}
