package vec_i32

import (
	"github.com/axieinfinity/steit-go/pkg/builtin/primitive"
	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/path"
	pathpkg "github.com/axieinfinity/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
	statepkg "github.com/axieinfinity/steit-go/pkg/state"
)

var _ statepkg.IState = (*VecI32)(nil)

type VecI32 struct {
	path  *pathpkg.Path
	Items []int32
	count int
}

func NewVec(path *pathpkg.Path, items []int32) *VecI32 {
	vector := &VecI32{}

	if path != nil {
		vector.path = path
	} else {
		vector.path = pathpkg.Root
	}
	if items == nil {
		vector.Items = make([]int32, 0)
	} else {
		vector.Items = items
		vector.count = len(items)
	}

	return vector
}

func (v *VecI32) Deserialize(reader readerpkg.IReader, path *pathpkg.Path) error {
	if path == nil {
		path = pathpkg.Root
	}

	var items []int32
	tag := uint32(0)

	for !readerpkg.EndOfStream(reader) {
		tag = tag + 1

		var value primitive.Int32
		err := statepkg.DeserializeNested(&value, reader, path, tag)
		if err != nil {
			return err
		}

		items = append(items, int32(value))
	}
	*v = *NewVec(path, items)
	return nil
}

func (v *VecI32) GetPath() *path.Path {
	return v.path
}

func (v *VecI32) GetWireType(tag uint32) *codec.WireType {
	return nil
}

func (v *VecI32) GetNested(tag uint32) statepkg.IState {
	return nil
}

func (v *VecI32) ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool) {
	if int(tag) >= v.count {
		panic("index out of range")
	}
	panic("not supported")
}

func (v *VecI32) ReplayListPush(reader readerpkg.IReader) {
	panic("not supported")
}

func (v *VecI32) ReplayListPop() {
	panic("not supported")
}

func (v *VecI32) ReplayMapRemove(key uint32) {
	panic("not supported")
}
