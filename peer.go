package main

import "strconv" // string conversion library

// Specifies connection type of a peer
// similar to an enum
type PeerType int

const (
	Direct PeerType = iota
	InDirect
)

// Represents known peer nodes
type Peer struct {
	// uint16 instead of uint8 to prevent JsonMarshal from treating ip as a string
	Ip          []uint16
	Port        uint32
	IsConnected bool
	Connection  PeerType
}

// These funcs act as a <obj>.url() style function
func (p *Peer) url(path string) string {
	var ipString string

	// iterates over each number in peer's Ip
	for index, element := range p.Ip {
		// casts the number and adds it to the ip string
		ipString = ipString + strconv.FormatUint(uint64(element), 10)

		// if its not the last number it adds a . to the ip string
		if index != len(p.Ip)-1 {
			ipString = ipString + "."
		}
	}

	// should be self explanatory imo
	return "ws://" + ipString + ":" + strconv.FormatUint(uint64(p.Port), 10) + "/" + path
}
