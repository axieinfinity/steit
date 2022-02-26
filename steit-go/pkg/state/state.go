package state

import (
	"log"

	"github.com/axieinfinity/steit/steit-go/pkg/codec"
	"github.com/axieinfinity/steit/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit/steit-go/pkg/reader"
)

type IState interface {
	GetPath() *path.Path
	GetWireType(uint32) *codec.WireType
	GetNested(uint32) IState

	ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool)
	ReplayListPush(reader readerpkg.IReader)
	ReplayListPop()
	ReplayMapRemove(uint32)
	Deserializer
}

type State struct {
	path     *path.Path
	wireType *codec.WireType
}

func (s *State) GetPath() *path.Path {
	return s.path
}

func (s *State) GetWireType() *codec.WireType {
	return s.wireType
}

func GetNested(state IState, path []uint32) IState {
	for _, tag := range path {
		state = state.GetNested(tag)
	}

	return state
}

func Replace(state IState, reader readerpkg.IReader, shouldNotify bool) {
	switch v := state.(type) {
	case IEnumState:
		tag := readerpkg.ReadUInt32(reader)
		v.ReplaceAt(tag, codec.WireTypeSized, reader, shouldNotify)
		return
	default:
		for !readerpkg.EndOfStream(reader) {
			tag, wireType := readerpkg.ReadKey(reader)
			expectedWireType := state.GetWireType(tag)
			var fieldReader readerpkg.IReader
			if wireType == codec.WireTypeSized {
				fieldReader = readerpkg.GetNested(reader)
			} else {
				fieldReader = reader
			}

			if expectedWireType != nil && wireType != *expectedWireType {
				var path = state.GetPath().GetNested(tag)
				log.Printf("Expected wire type %v for path %v, got %v.\n", *expectedWireType, path, wireType)
				readerpkg.SkipField(fieldReader, wireType)
				continue
			}

			state.ReplaceAt(tag, wireType, fieldReader, shouldNotify)
		}
	}
}
