package none

import (
	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/eventhandler"
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

func Deserialize(reader readerpkg.IReader, path *pathpkg.Path) *None {
	none := NewNone(path)
	statepkg.Replace(none, reader, false)
	return none
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
	reader.SkipField(wireType)
}

func (n *None) ReplayListPush(reader readerpkg.IReader) { panic("not supported") }
func (n *None) ReplayListPop()                          { panic("not supported") }
func (n *None) ReplayMapRemove(uint32)                  { panic("not supported") }

func (n *None) MaybeNotify(
	tag uint32,
	newValue interface{},
	oldValue interface{},
	handler eventhandler.EventHandler,
	shouldNotify bool,
) interface{} {
	if shouldNotify {
		var args = NewFieldUpdateEventArgs(tag, newValue, oldValue, n)
		if handler != nil {
			handler(n, args)
		}
	}

	return newValue
}
