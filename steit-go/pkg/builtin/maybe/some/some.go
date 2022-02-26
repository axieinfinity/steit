package some

import (
	"log"
	"reflect"

	"github.com/axieinfinity/steit/steit-go/pkg/codec"
	"github.com/axieinfinity/steit/steit-go/pkg/event"
	pathpkg "github.com/axieinfinity/steit/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit/steit-go/pkg/reader"
	statepkg "github.com/axieinfinity/steit/steit-go/pkg/state"

	"github.com/axieinfinity/steit/steit-go/pkg/utils"
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

func (s *Some) Deserialize(reader readerpkg.IReader, path *pathpkg.Path) error {
	some := NewSome(path, s._type)
	statepkg.Replace(some, reader, false)
	*s = *some
	return nil
}

func (s *Some) GetWireType(tag uint32) *codec.WireType {
	switch tag {
	case 0:
		if statepkg.IsStateType(reflect.TypeOf(s.f0)) {
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

func (s *Some) GetNested(tag uint32) statepkg.IState {
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
		err := statepkg.Deserialize(s.f0.(statepkg.Deserializer), reader, s.path, statepkg.DeserializeWithTag(0))
		if err != nil {
			readerpkg.SkipToEnd(reader)
			log.Println("parse some error:", err)
		}
	default:
		readerpkg.SkipField(reader, wireType)
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
