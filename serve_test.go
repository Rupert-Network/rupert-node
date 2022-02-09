package main

import (
	"net/http/httptest"
	"net/rpc/jsonrpc"
	"strings"
	"testing"

	"golang.org/x/net/websocket"
)

func TestRPCHandler(t *testing.T) {
	// Start test http server
	s := httptest.NewServer(websocket.Handler(RPCHandler))
	defer s.Close() // Close server after func

	// Extract test server url and replace http with ws
	u := "ws" + strings.TrimPrefix(s.URL, "http")

	// Start websocket connection
	ws, err := websocket.Dial(u, "", "http://localhost/")
	if err != nil {
		t.Fatalf("in DefaultDieler %v", err)
	}
	defer ws.Close() // Close websocket after func

	// Create args
	// A: int, B: int
	args := struct{ A, B int }{7, 8}
	var reply int

	// Create rpc client
	c := jsonrpc.NewClient(ws)

	// Call Arith.Multiply(args, &reply)
	err = c.Call("Arith.Multiply", args, &reply)
	if err != nil {
		t.Fatal("arith error:", err)
	}

	if reply != 56 {
		t.Fatalf("expected %d got %d", 56, reply)
	}
}
