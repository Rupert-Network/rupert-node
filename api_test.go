package main

import (
	"context"
	"encoding/json"
	"log"
	"net/http"
	"net/http/httptest"
	"sync"
	"testing"

	"github.com/go-redis/redismock/v8"
)

type Peers struct {
	peers []Peer
}

func (p *Peers) PeerList(max int) []Peer {
	return p.peers[0:max]
}

var ctx = context.Background()

func TestListPeerHandler(t *testing.T) {
	req, err := http.NewRequest("GET", "/list_peers", nil) // Create new request
	q := req.URL.Query()

	// /list_peers?start=0&stop=2
	q.Add("start", "0")
	q.Add("stop", "2")
	req.URL.RawQuery = q.Encode()

	if err != nil {
		t.Fatal(err)
	}
	rr := httptest.NewRecorder() // Create response recorder

	var pl = []Peer{ // Create initial peer list
		Peer{[]uint16{192, 168, 1, 4}, 3030, false, Direct},
		Peer{[]uint16{192, 168, 2, 3}, 3030, false, InDirect},
		Peer{[]uint16{192, 168, 3, 2}, 3030, false, Direct},
		Peer{[]uint16{192, 168, 4, 1}, 3030, false, InDirect},
	}

	peerKeys := []string{"PEER0", "PEER1", "PEER2", "PEER3"}

	db, mock := redismock.NewClientMock()

	var start int64
	var stop int64 = 2
	mock.ExpectLRange("PEER_LIST", start, stop).SetVal(peerKeys[start:stop])

	for i := start; i < stop; i++ {
		jsonPeer, err := json.Marshal(pl[i])
		if err != nil {
			t.Errorf("error marshaling: %s", err.Error())
		}
		mock.ExpectGet(peerKeys[i]).SetVal(string(jsonPeer))
	}

	rb := NewRedisBackend(db)

	handler := http.HandlerFunc(ListPeersHandler(&sync.Mutex{}, rb))
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("status code: got %v wanted %v", status, http.StatusOK)
	}

	peerSlice := pl[0:2]

	jsonPeers, err := json.Marshal(peerSlice)
	if err != nil {
		// prints out an error
		log.Fatalf("Error occured during marshaling. Error: %s", err.Error())
	}

	expected := string(jsonPeers)

	if rr.Body.String() != expected {
		t.Errorf("body: got \n%v\n wanted \n%v\n", rr.Body.String(), expected)
	}
}
