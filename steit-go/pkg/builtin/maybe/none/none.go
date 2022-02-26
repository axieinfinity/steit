package none

import (
	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/event"
	pathpkg "github.com/axieinfinity/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
	statepkg "github.com/axieinfinity/steit-go/pkg/state"
)

var _ statepkg.IState = (*None)(nil)

type None struct {
	path *pathpkg.Path
}

func NewNone(path *pathpkg.Path) *None {
	none := None{}

	if path != nil {
		none.path = path
	} else {
		none.path = pathpkg.Root
	}

	return &none
}

func (n *None) ClearUpdateHandlers() {}

func (n *None) Deserialize(reader readerpkg.IReader, path *pathpkg.Path) error {
	none := NewNone(path)
	statepkg.Replace(none, reader, false)
	*n = *none
	return nil
}

func (n *None) GetPath() *pathpkg.Path {
	return n.path
}

func (n *None) GetWireType(tag uint32) *codec.WireType {
	return nil
}

func (n *None) GetNested(tag uint32) statepkg.IState {
	return nil
}

func (n *None) ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool) {
	readerpkg.SkipField(reader, wireType)
}

func (n *None) ReplayListPush(reader readerpkg.IReader) { panic("not supported") }
func (n *None) ReplayListPop()                          { panic("not supported") }
func (n *None) ReplayMapRemove(uint32)                  { panic("not supported") }

func (n *None) MaybeNotify(
	tag uint32,
	newValue interface{},
	oldValue interface{},
	handler event.EventHandler,
	shouldNotify bool,
) interface{} {

	return newValue
}
