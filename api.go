package main

import (
	"encoding/json" // json serialization lib (rust -> serde_json, serde)
	"fmt"           // io lib
	"log"           // logging  lib
	"net/http"      // networking lib (think warp rs)
	"strconv"
	"sync" // used for syncronization between async processes
)

// PeerLister ...
type PeerLister interface {
	PeerList(start, stop int64) []Peer
}

// ListPeersHandler ...
func ListPeersHandler(m *sync.Mutex, pl PeerLister) func(http.ResponseWriter, *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		m.Lock()
		defer m.Unlock()

		// Get max num of peers
		start, err := strconv.ParseInt(r.URL.Query().Get("start"), 10, 64)
		if err != nil {
			start = 0
		}

		stop, err := strconv.ParseInt(r.URL.Query().Get("stop"), 10, 64)
		if err != nil {
			stop = 0
		}

		jsonPeers, err := json.Marshal(pl.PeerList(start, stop))
		if err != nil {
			// prints out an error
			log.Fatalf("Error occured during marshaling. Error: %s", err.Error())
		}

		fmt.Fprintf(w, "%s", string(jsonPeers))
	}
}
