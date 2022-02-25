package maybe

import (
	"log"
	"reflect"

	"github.com/axieinfinity/steit-go/pkg/builtin/maybe/none"
	"github.com/axieinfinity/steit-go/pkg/builtin/maybe/some"
	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/eventhandler"
	"github.com/axieinfinity/steit-go/pkg/path"
	pathpkg "github.com/axieinfinity/steit-go/pkg/path"
	"github.com/axieinfinity/steit-go/pkg/reader"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
	"github.com/axieinfinity/steit-go/pkg/state"
	statepkg "github.com/axieinfinity/steit-go/pkg/state"
)

const (
	NoneTag = 0
	SomeTag = 1
)

var _ statepkg.IEnumState = (*Maybe)(nil)

type Maybe struct {
	path        *pathpkg.Path
	tag         uint32
	variant     statepkg.IState
	_type       reflect.Type
	noneVariant *none.None
	someVariant *some.Some
	onUpdate    eventhandler.EventHandler
}

func NewMaybe(_type reflect.Type, path *path.Path) *Maybe {
	maybe := &Maybe{}
	err := statepkg.ValidateType(_type)
	if err != nil {
		log.Fatalln("newmaybe failed, error", err.Error())
		return nil
	}
	if path != nil {
		maybe.path = path
	} else {
		maybe.path = pathpkg.Root
	}

	maybe.tag = 0
	maybe.variant = none.NewNone(maybe.path.GetNested(0))

	return maybe
}

func (mb *Maybe) ClearUpdateHandlers() {
	mb.onUpdate = nil
}

func Deserialize(_type reflect.Type, reader readerpkg.IReader, path *pathpkg.Path) *Maybe {
	maybe := NewMaybe(_type, path)
	statepkg.Replace(maybe, reader, false)
	return maybe
}

func (mb *Maybe) GetPath() *path.Path {
	return mb.path
}

func (mb *Maybe) GetWireType(tag uint32) *codec.WireType {
	switch tag {
	case 0:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	case 1:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	default:
		return nil
	}
}

func (mb *Maybe) GetNested(tag uint32) state.IState {
	if tag == mb.tag {
		return mb.variant
	} else {
		return nil
	}
}

func (mb *Maybe) ReplaceAt(tag uint32, wireType codec.WireType, reader reader.IReader, shouldNotify bool) {
	switch tag {
	case 0:
		mb.UpdateAndNotify(0, none.Deserialize(reader, mb.path.GetNested(0)), shouldNotify)
	case 1:
		mb.UpdateAndNotify(1, some.Deserialize(mb._type, reader, mb.path.GetNested(1)), shouldNotify)
	default:
		readerpkg.SkipToEnd(reader)
	}
}

func (mb *Maybe) ReplayListPush(reader reader.IReader) {
	panic("not supported")
}

func (mb *Maybe) ReplayListPop() {
	panic("not supported")
}

func (mb *Maybe) ReplayMapRemove(_ uint32) {
	panic("not supported")
}

func (mb *Maybe) UpdateAndNotify(newTag uint32, newVariant state.IState, shouldNotify bool) {
	if shouldNotify {
		args := NewVariantUpdateEventArgs(newTag, newVariant, mb.tag, mb.variant, mb)
		if mb.onUpdate != nil {
			mb.onUpdate(mb, args)
		}
	}

	mb.tag = newTag
	mb.variant = newVariant
}
