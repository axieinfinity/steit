package collections

import (
	"reflect"

	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/eventhandler"
	pathpkg "github.com/axieinfinity/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
	statepkg "github.com/axieinfinity/steit-go/pkg/state"
)

type StateList struct {
	Path     *pathpkg.Path
	Items    []interface{}
	Count    int
	OnUpdate eventhandler.EventHandler
	OnPush   eventhandler.EventHandler
	OnPop    eventhandler.EventHandler
}

func NewStateList(path *pathpkg.Path, items []interface{}) StateList {
	stateList := StateList{}

	if path != nil {
		stateList.Path = path
	} else {
		stateList.Path = pathpkg.Root
	}

	if len(items) > 0 {
		stateList.Items = items
		stateList.Count = len(items)
	}

	return stateList
}

func (s *StateList) ClearUpdateHandlers() {
	s.OnUpdate = nil
}

func (s *StateList) ClearPushHandlers() {
	s.OnPush = nil
}

func (s *StateList) ClearPopHandlers() {
	s.OnPop = nil
}

func Deserialize(reader readerpkg.IReader, path *pathpkg.Path) StateList {
	if path == nil {
		path = pathpkg.Root
	}

	var items []interface{}
	tag := uint32(0)

	for !reader.EndOfStream() {
		tag = tag + 1
		items = append(items, readerpkg.ReadValue(reader, path, tag))
	}

	return NewStateList(path, items)
}

func (s *StateList) GetWireType(tag uint32) *codec.WireType {
	if statepkg.IsStateType(reflect.TypeOf(s.Items).Elem()) {
		c := codec.WireTypeSized
		return &c
	} else {
		c := codec.WireTypeVarint
		return &c
	}
}

func (s *StateList) GetNested(tag uint32) statepkg.IState {
	if int(tag) < s.Count {
		if value, ok := s.Items[tag].(statepkg.IState); !ok {
			panic("item not istate type")
		} else {
			return value
		}
		return s.Items[tag].(statepkg.IState)
	} else {
		return nil
	}
}

func (s *StateList) ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool) {
	if int(tag) >= s.Count {
		panic("index out of range")
	}

	newItem := statepkg.Deserialize(reader, s.Path, statepkg.DeserializeWithTag(tag))
	oldItem := s.Items[tag]

	if shouldNotify {
		args := NewFieldUpdateEventArgs(tag, newItem, oldItem, s)
		if s.OnUpdate != nil {
			s.OnUpdate(s, args)
		}
	}

	s.Items[tag] = newItem
}

func (s *StateList) ReplayListPush(reader readerpkg.IReader) {
	tag := uint32(s.Count)
	item := statepkg.Deserialize(reader, s.Path, tag)

	args := NewListPushEventArgs(tag, item, s)
	if s.OnPush != nil {
		s.OnPush(s, args)
	}

	s.Items = append(s.Items, item)
}

func (s *StateList) ReplayListPop() {
	if s.Count <= 0 {
		panic("Cannot pop from an empty `StateList`.")
	}

	tag := uint32(s.Count - 1)

	args := NewListPopEventArgs(tag, s.Items[tag], s)
	if s.OnPop != nil {
		s.OnPop(s, args)
	}

	s.Items = append(s.Items[:tag], s.Items[tag+1:]...)
}

func (s *StateList) ReplayMapRemove(key uint32) {
	panic("not supported")
}
