package state

import (
	"log"

	"github.com/axieinfinity/steit-go/pkg/codec"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
)

type IState interface {
	GetPath() *codec.Path
	GetWireType(uint32) *codec.WireType
	GetNested(uint32) IState

	ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool)
	ReplayListPush(reader readerpkg.IReader)
	ReplayListPop()
	ReplayMapRemove(uint32)
}

type State struct {
	path     *codec.Path
	wireType *codec.WireType
}

func (s *State) GetPath() *codec.Path {
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
		variant := reader.ReadUint32()
		v.ReplaceAt(variant, codec.WireTypeSized, reader, shouldNotify)
		return
	default:
		for !reader.EndOfStream() {
			tag, wireType := reader.ReadKey()
			expectedWireType := state.GetWireType(tag)
			var fieldReader readerpkg.IReader
			if wireType == codec.WireTypeSized {
				fieldReader = reader.GetNested()
			} else {
				fieldReader = reader
			}

			if expectedWireType != nil && wireType != *expectedWireType {
				var path = state.GetPath().GetNested(tag)
				log.Printf("Expected wire type %v for path %v, got %v.\n", *expectedWireType, path, wireType)
				fieldReader.SkipField(wireType)
				continue
			}

			state.ReplaceAt(tag, wireType, fieldReader, shouldNotify)
		}
	}
}
