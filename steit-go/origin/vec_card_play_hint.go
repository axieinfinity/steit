package origin

import (
	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/path"
	pathpkg "github.com/axieinfinity/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
	statepkg "github.com/axieinfinity/steit-go/pkg/state"
)

var _ statepkg.IState = (*VecCardPlayHint)(nil)

type VecCardPlayHint struct {
	path  *pathpkg.Path
	items []*CardPlayHint
	count int
}

func NewVecCardPlayHint(path *pathpkg.Path, items []*CardPlayHint) *VecCardPlayHint {
	vector := &VecCardPlayHint{}

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

func (s *VecCardPlayHint) Deserialize(reader readerpkg.IReader, path *pathpkg.Path) error {
	if path == nil {
		path = pathpkg.Root
	}

	var items []*CardPlayHint
	tag := uint32(0)

	for !readerpkg.EndOfStream(reader) {
		tag = tag + 1
		item := NewCardPlayHint(path)
		err := statepkg.DeserializeNested(item, reader, path, tag)
		if err != nil {
			return err
		}
		items = append(items, item)
	}

	*s = *NewVecCardPlayHint(path, items)

	return nil
}

func (v *VecCardPlayHint) GetPath() *path.Path {
	return v.path
}

func (v *VecCardPlayHint) GetWireType(tag uint32) *codec.WireType {
	return nil
}

func (v *VecCardPlayHint) GetNested(tag uint32) statepkg.IState {
	return nil
}

func (v *VecCardPlayHint) ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool) {
	if int(tag) >= v.count {
		panic("index out of range")
	}
	panic("not supported")
}

func (v *VecCardPlayHint) ReplayListPush(reader readerpkg.IReader) {
	panic("not supported")
}

func (v *VecCardPlayHint) ReplayListPop() {
	panic("not supported")
}

func (v *VecCardPlayHint) ReplayMapRemove(key uint32) {
	panic("not supported")
}
