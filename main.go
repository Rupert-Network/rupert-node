package main

import (
	"fmt"
	"log"
	"net/http"
	"sync"
)

// create new state with array of peers
var state = &State{&sync.Mutex{}, []Peer{
	Peer{[]uint16{192, 168, 1, 4}, 3030, false, Direct},
	Peer{[]uint16{192, 168, 2, 3}, 3030, false, InDirect},
	Peer{[]uint16{192, 168, 3, 2}, 3030, false, Direct},
	Peer{[]uint16{192, 168, 4, 1}, 3030, false, InDirect},
}}

func main() {
	// creates and runs new endpoint /list_peers where all
	// http requests will be routed to ListPeersHandler that has access
	// to state
	http.HandleFunc("/list_peers", ListPeersHandler(state))

	fmt.Println("Server started at port 8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
