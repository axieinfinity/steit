package statelist

import (
	"log"
	"reflect"

	"github.com/axieinfinity/steit/steit-go/pkg/codec"
	"github.com/axieinfinity/steit/steit-go/pkg/path"
	pathpkg "github.com/axieinfinity/steit/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit/steit-go/pkg/reader"
	statepkg "github.com/axieinfinity/steit/steit-go/pkg/state"
)

var _ statepkg.IState = (*StateList)(nil)

type StateList struct {
	path  *pathpkg.Path
	items []interface{}
	_type reflect.Type
	count int
}

func NewStateList(path *pathpkg.Path, items []interface{}) StateList {
	stateList := StateList{}

	if path != nil {
		stateList.path = path
	} else {
		stateList.path = pathpkg.Root
	}

	if items != nil {
		stateList.items = items
		stateList.count = len(items)
		stateList._type = reflect.TypeOf(items).Elem()
	}

	return stateList
}

func (s *StateList) GetItems() []interface{} {
	return s.items
}

func (s *StateList) GetCount() int {
	return s.count
}

func (s *StateList) Deserialize(r readerpkg.IReader, path *pathpkg.Path) error {
	if path == nil {
		path = pathpkg.Root
	}

	var items []interface{}
	tag := uint32(0)

	for !readerpkg.EndOfStream(r) {
		tag = tag + 1
		val := reflect.New(s._type).Interface().(statepkg.Deserializer)
		statepkg.DeserializeNested(val, r, path, tag)
		items = append(items)
	}
	*s = NewStateList(path, items)
	return nil
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
	} else {
		return nil
	}
}

func (s *StateList) ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool) {
	if int(tag) >= s.count {
		panic("index out of range")
	}

	newItem := reflect.New(s._type).Interface().(statepkg.Deserializer)
	err := statepkg.Deserialize(newItem, reader, s.path, statepkg.DeserializeWithTag(tag))
	if err != nil {
		readerpkg.SkipToEnd(reader)
		log.Println("parse statelist error", err)
	} else {
		s.items[tag] = newItem
	}
}

func (s *StateList) ReplayListPush(reader readerpkg.IReader) {
	tag := uint32(s.count)
	newItem := reflect.New(s._type).Interface().(statepkg.Deserializer)
	err := statepkg.Deserialize(newItem, reader, s.path, statepkg.DeserializeWithTag(tag))
	if err != nil {
		panic(err)
	}

	s.items = append(s.items, newItem)
}

func (s *StateList) ReplayListPop() {
	if s.count <= 0 {
		panic("Cannot pop from an empty `StateList`.")
	}

	tag := uint32(s.count - 1)

	s.items = append(s.items[:tag], s.items[tag+1:]...)
}

func (s *StateList) ReplayMapRemove(key uint32) {
	panic("not supported")
}

func (s *StateList) GetPath() *path.Path {
	return s.path
}
