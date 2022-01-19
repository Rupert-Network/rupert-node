package main

import (
	"encoding/json" // json serialization lib (rust -> serde_json, serde)
	"fmt"           // io lib
	"log"           // logging  lib
	"net/http"      // networking lib (think warp rs)
	"sync"          // used for syncronization between async processes
)

// Standard data stuct
type State struct {
	// Makes it to where only one thing can access the shared state
	*sync.Mutex
	// Current state can be anything
	CurrentState interface{}
}

// Takes a reference to a state and returns a handler func
func ListPeersHandler(s *State) func(http.ResponseWriter, *http.Request) {
	// Takes a response writer (where you send data) and an http request
	return func(w http.ResponseWriter, r *http.Request) {
		// Locks access to state
		s.Lock()
		// Unlocks access at end of function
		defer s.Unlock()

		// One of either jsonPeers or err will be nil
		// jsonPeer will be nil if the function fails and err wont be nil
		// jsonPeer will not be nil if the function succeeds and err will be nil
		jsonPeers, err := json.Marshal(s.CurrentState)
		if err != nil {
			// prints out an error
			log.Fatalf("Error occured during marshaling. Error: %s", err.Error())
		}

		// treats the response writer as a place you can write data to (like standard io)
		// converts jsonPeers to a string
		fmt.Fprintf(w, "%s\n", string(jsonPeers))

		// sets the current state to an empty array of peers
		s.CurrentState = []Peer{}
	}
}
