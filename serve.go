package main

import (
	"net/http"
	"net/rpc/jsonrpc"

	"golang.org/x/net/websocket"
)

// Serve ...
func Serve() {
	http.Handle("/rpc", websocket.Handler(RPCHandler))
}

// RPCHandler ...
func RPCHandler(ws *websocket.Conn) {
	RegisterRPC()
	jsonrpc.ServeConn(ws)
}
