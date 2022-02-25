package vec_u32

import (
	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/path"
	pathpkg "github.com/axieinfinity/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
	statepkg "github.com/axieinfinity/steit-go/pkg/state"
)

var _ statepkg.IState = (*VecU32)(nil)

type VecU32 struct {
	path  *pathpkg.Path
	items []uint32
	count int
}

func NewVecU32(path *pathpkg.Path, items []uint32) *VecU32 {
	vector := &VecU32{}

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

func Deserialize(reader readerpkg.IReader, path *pathpkg.Path) *VecU32 {
	if path == nil {
		path = pathpkg.Root
	}

	var items []uint32
	tag := uint32(0)

	for !reader.EndOfStream() {
		tag = tag + 1
		items = append(items, readerpkg.ReadValue(reader, path, tag))
	}

	return NewVecU32(path, items)
}

func (v *VecU32) GetPath() *path.Path {
	return v.path
}

func (v *VecU32) GetWireType(tag uint32) *codec.WireType {
	return nil
}

func (v *VecU32) GetNested(tag uint32) statepkg.IState {
	return nil
}

func (v *VecU32) ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool) {
	if int(tag) >= v.count {
		panic("index out of range")
	}
	panic("not supported")
}

func (v *VecU32) ReplayListPush(reader readerpkg.IReader) {
	panic("not supported")
}

func (v *VecU32) ReplayListPop() {
	panic("not supported")
}

func (v *VecU32) ReplayMapRemove(key uint32) {
	panic("not supported")
}