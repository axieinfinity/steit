package some

import (
	"reflect"

	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/event"

	pathpkg "github.com/axieinfinity/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
	"github.com/axieinfinity/steit-go/pkg/state"
	statepkg "github.com/axieinfinity/steit-go/pkg/state"
	"github.com/axieinfinity/steit-go/pkg/utils"
)

var _ statepkg.IState = (*Some)(nil)

type Some struct {
	path  *pathpkg.Path
	f0    interface{}
	_type reflect.Type
}

func NewSome(path *pathpkg.Path, _type reflect.Type) *Some {
	some := &Some{}

	if path != nil {
		some.path = path
	} else {
		some.path = pathpkg.Root
	}

	some.f0 = statepkg.Construct(_type, some.path.GetNested(0))

	return some
}

func Deserialize(_type reflect.Type, reader readerpkg.IReader, path *pathpkg.Path) *Some {
	some := NewSome(path, _type)
	statepkg.Replace(some, reader, false)
	return some
}

func (s *Some) GetWireType(tag uint32) *codec.WireType {
	switch tag {
	case 0:
		if state.IsStateType(reflect.TypeOf(s.f0)) {
			return codec.GetWireTypePtr(codec.WireTypeSized)
		} else {
			return codec.GetWireTypePtr(codec.WireTypeVarint)
		}
	default:
		return nil
	}
}

func (s *Some) GetPath() *pathpkg.Path {
	return s.path
}

func (s *Some) GetNested(tag uint32) state.IState {
	switch tag {
	case 0:
		return utils.AsIState(s.f0)
	default:
		return nil
	}
}

func (s *Some) ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool) {
	switch tag {
	case 0:
		s.f0 = s.MaybeNotify(0, statepkg.Deserialize(s._type, reader, s.path, statepkg.DeserializeWithTag(0)), s.f0, nil, shouldNotify)
	default:
		reader.SkipField(wireType)
	}
}

func (s *Some) ReplayListPush(reader readerpkg.IReader) { panic("not supported") }
func (s *Some) ReplayListPop()                          { panic("not supported") }
func (s *Some) ReplayMapRemove(uint32)                  { panic("not supported") }

func (s *Some) MaybeNotify(
	tag uint32,
	newValue interface{},
	oldValue interface{},
	handler event.EventHandler,
	shouldNotify bool,
) interface{} {

	return newValue
}
