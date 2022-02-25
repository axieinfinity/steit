package reader

import "github.com/axieinfinity/steit-go/pkg/codec"

type IReader interface {
	Remaining() int
	ReadUint8() byte
	Read(int) []byte
	Skip(int)
}
