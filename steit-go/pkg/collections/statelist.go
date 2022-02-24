package collections

import (
	"reflect"

	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/eventhandler"
	"github.com/axieinfinity/steit-go/pkg/path"
	pathpkg "github.com/axieinfinity/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
	statepkg "github.com/axieinfinity/steit-go/pkg/state"
)

var _ statepkg.IState = (*StateList)(nil)

type StateList struct {
	path     *pathpkg.Path
	items    []interface{}
	count    int
	OnUpdate eventhandler.EventHandler
	OnPush   eventhandler.EventHandler
	OnPop    eventhandler.EventHandler
}

func NewStateList(path *pathpkg.Path, items []interface{}) StateList {
	stateList := StateList{}

	if path != nil {
		stateList.path = path
	} else {
		stateList.path = pathpkg.Root
	}

	if len(items) > 0 {
		stateList.items = items
		stateList.count = len(items)
	}

	return stateList
}

func (s *StateList) GetItems() []interface{} {
	return s.items
}

func (s *StateList) GetCount() int {
	return s.count
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
	if statepkg.IsStateType(reflect.TypeOf(s.items).Elem()) {
		c := codec.WireTypeSized
		return &c
	} else {
		c := codec.WireTypeVarint
		return &c
	}
}

func (s *StateList) GetNested(tag uint32) statepkg.IState {
	if int(tag) < s.count {
		if value, ok := s.items[tag].(statepkg.IState); !ok {
			panic("item not istate type")
		} else {
			return value
		}
		return s.items[tag].(statepkg.IState)
	} else {
		return nil
	}
}

func (s *StateList) ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool) {
	if int(tag) >= s.count {
		panic("index out of range")
	}

	newItem := statepkg.Deserialize(reader, s.path, statepkg.DeserializeWithTag(tag))
	oldItem := s.items[tag]

	if shouldNotify {
		args := NewFieldUpdateEventArgs(tag, newItem, oldItem, s)
		if s.OnUpdate != nil {
			s.OnUpdate(s, args)
		}
	}

	s.items[tag] = newItem
}

func (s *StateList) ReplayListPush(reader readerpkg.IReader) {
	tag := uint32(s.count)
	item := statepkg.Deserialize(reader, s.path, statepkg.DeserializeWithTag(tag))

	args := NewListPushEventArgs(tag, item, s)
	if s.OnPush != nil {
		s.OnPush(s, args)
	}

	s.items = append(s.items, item)
}

func (s *StateList) ReplayListPop() {
	if s.count <= 0 {
		panic("Cannot pop from an empty `StateList`.")
	}

	tag := uint32(s.count - 1)

	args := NewListPopEventArgs(tag, s.items[tag], s)
	if s.OnPop != nil {
		s.OnPop(s, args)
	}

	s.items = append(s.items[:tag], s.items[tag+1:]...)
}

func (s *StateList) ReplayMapRemove(key uint32) {
	panic("not supported")
}

func (s *StateList) GetPath() *path.Path {
	return s.path
}
