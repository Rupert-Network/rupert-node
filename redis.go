package main

import (
	"context"
	"encoding/json"
	"log"

	"github.com/go-redis/redis/v8"
)

// RedisBackend is ...
type RedisBackend struct {
	db *redis.Client
}

// NewRedisBackend ...
func NewRedisBackend(db *redis.Client) RedisBackend {
	return RedisBackend{db}
}

// PeerList ...
func (rb RedisBackend) PeerList(start, stop int64) []Peer {
	var ctx = context.Background()
	log.Printf("start: %d, stop: %d", start, stop)
	peers := []Peer{}

	// Get list of peers as keys
	keyList := "PEER_LIST"
	peersStrings, err := rb.db.LRange(ctx, keyList, start, stop).Result()
	if err != nil {
		log.Fatal(err)
		return peers
	}
	if len(peersStrings) < 1 {
		log.Print("no peers handed")
		return peers
	}

	// Fetch each peer from db and deserialize into Peer
	// then add to list
	var peer Peer
	for _, pk := range peersStrings {
		peer = Peer{}
		peerString, err := rb.db.Get(ctx, pk).Result()
		if err != nil {
			log.Fatal(err)
			return peers
		}

		err = json.Unmarshal([]byte(peerString), &peer)
		if err != nil {
			log.Fatalf("Error unmarshaling: %s", err.Error())
			return peers
		}
		peers = append(peers, peer)
	}

	return peers
}
