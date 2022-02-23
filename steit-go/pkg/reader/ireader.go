package reader

import "github.com/axieinfinity/steit-go/pkg/codec"

type IReader interface {
	ReadUint32() uint32
	EndOfStream() bool
	ReadKey() (uint32, codec.WireType)
	GetNested() IReader
	SkipField(int)
}
