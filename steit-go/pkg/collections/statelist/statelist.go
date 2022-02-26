package statelist

// import (
// 	"reflect"

// 	"github.com/axieinfinity/steit-go/pkg/codec"
// 	"github.com/axieinfinity/steit-go/pkg/path"
// 	pathpkg "github.com/axieinfinity/steit-go/pkg/path"
// 	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
// 	statepkg "github.com/axieinfinity/steit-go/pkg/state"
// )

// var _ statepkg.IState = (*StateList)(nil)

// type StateList struct {
// 	path  *pathpkg.Path
// 	items []interface{}
// 	_type reflect.Type
// 	count int
// }

// func NewStateList(path *pathpkg.Path, items []interface{}) StateList {
// 	stateList := StateList{}

// 	if path != nil {
// 		stateList.path = path
// 	} else {
// 		stateList.path = pathpkg.Root
// 	}

// 	if items != nil {
// 		stateList.items = items
// 		stateList.count = len(items)
// 		stateList._type = reflect.TypeOf(items).Elem()
// 	}

// 	return stateList
// }

// func (s *StateList) GetItems() []interface{} {
// 	return s.items
// }

// func (s *StateList) GetCount() int {
// 	return s.count
// }

// func Deserialize(r readerpkg.IReader, path *pathpkg.Path) StateList {
// 	if path == nil {
// 		path = pathpkg.Root
// 	}

// 	var items []interface{}
// 	tag := uint32(0)

// 	for !readerpkg.EndOfStream(r) {
// 		tag = tag + 1
// 		items = append(items, readerpkg.ReadValue(r, path, tag))
// 	}

// 	return NewStateList(path, items)
// }

// func (s *StateList) GetWireType(tag uint32) *codec.WireType {
// 	if statepkg.IsStateType(reflect.TypeOf(s.items).Elem()) {
// 		c := codec.WireTypeSized
// 		return &c
// 	} else {
// 		c := codec.WireTypeVarint
// 		return &c
// 	}
// }

// func (s *StateList) GetNested(tag uint32) statepkg.IState {
// 	if int(tag) < s.count {
// 		if value, ok := s.items[tag].(statepkg.IState); !ok {
// 			panic("item not istate type")
// 		} else {
// 			return value
// 		}
// 	} else {
// 		return nil
// 	}
// }

// func (s *StateList) ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool) {
// 	if int(tag) >= s.count {
// 		panic("index out of range")
// 	}

// 	newItem := statepkg.Deserialize(s._type, reader, s.path, statepkg.DeserializeWithTag(tag))
// 	_ = s.items[tag]

// 	s.items[tag] = newItem
// }

// func (s *StateList) ReplayListPush(reader readerpkg.IReader) {
// 	tag := uint32(s.count)
// 	item := statepkg.Deserialize(s._type, reader, s.path, statepkg.DeserializeWithTag(tag))

// 	s.items = append(s.items, item)
// }

// func (s *StateList) ReplayListPop() {
// 	if s.count <= 0 {
// 		panic("Cannot pop from an empty `StateList`.")
// 	}

// 	tag := uint32(s.count - 1)

// 	s.items = append(s.items[:tag], s.items[tag+1:]...)
// }

// func (s *StateList) ReplayMapRemove(key uint32) {
// 	panic("not supported")
// }

// func (s *StateList) GetPath() *path.Path {
// 	return s.path
// }
