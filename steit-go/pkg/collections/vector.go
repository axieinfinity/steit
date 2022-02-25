package collections

import (
	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/path"
	pathpkg "github.com/axieinfinity/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
	statepkg "github.com/axieinfinity/steit-go/pkg/state"
)

var _ statepkg.IState = (*Vector)(nil)

type Vector struct {
	path  *pathpkg.Path
	items []interface{}
	count int
}

func NewVector(path *pathpkg.Path, items []interface{}) *Vector {
	vector := &Vector{}

	if path != nil {
		vector.path = path
	} else {
		vector.path = pathpkg.Root
	}

	if len(items) > 0 {
		vector.items = items
		vector.count = len(items)
	}

	return vector
}

func (v *Vector) Deserialize(reader readerpkg.IReader, path *pathpkg.Path) *Vector {
	if path == nil {
		path = pathpkg.Root
	}

	var items []interface{}
	tag := uint32(0)

	for !reader.EndOfStream() {
		tag = tag + 1
		items = append(items, readerpkg.ReadValue(reader, path, tag))
	}

	return NewVector(path, items)
}

func (v *Vector) GetPath() *path.Path {
	return v.path
}

func (v *Vector) GetWireType(tag uint32) *codec.WireType {
	return nil
}

func (v *Vector) GetNested(tag uint32) statepkg.IState {
	return nil
}

func (v *Vector) ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool) {
	if int(tag) >= v.count {
		panic("index out of range")
	}
	panic("not supported")
}

func (v *Vector) ReplayListPush(reader readerpkg.IReader) {
	panic("not supported")
}

func (v *Vector) ReplayListPop() {
	panic("not supported")
}

func (v *Vector) ReplayMapRemove(key uint32) {
	panic("not supported")
}
