package maybe

import (
	pathpkg "github.com/axieinfinity/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
	"google.golang.org/grpc/balancer/grpclb/state"
	"reflect"
)

const (
	NoneTag = 0
	SomeTag = 1
)

//var _ state.IEnumState = (*Maybe)(nil)

type Maybe struct {
	path        *pathpkg.Path
	tag         uint32
	variant     state.IState
	noneVariant *None
	someVariant *Some
	onUpdate    *EventHandler
}

func NewMaybe(path *path.Path) Maybe {
	maybe := Maybe{}

	if path != nil {
		maybe.path = path
	} else {
		maybe.path = pathpkg.Root
	}

	maybe.tag = 0
	maybe.variant = NewNone(maybe.path.GetNested(0))

	return maybe
}

func (mb *Maybe) ClearUpdateHandlers() {
	mb.onUpdate = nil
}

func (mb *Maybe) Deserialize(reader readerpkg.IReader, path *pathpkg.Path) Maybe {
	maybe := NewMaybe(path)
	maybe.Replace(reader, false)
	return maybe
}

func (mb *Maybe) GetWireType(tag uint32) *codec.WireType {
	switch tag {
	case 0: return WireTypeSized
	case 1: return WireTypeSized
	default: return nil
	}
}

func (mb *Maybe) GetNested(tag uint32) *state.IState {
	if tag == mb.tag {
		return mb.variant
	} else {
		return nil
	}
}

func (mb *Maybe) ReplaceAt(tag uint32, wireType codec.WireType, reader reader.IReader, shouldNotify bool) {
	switch tag {
	case 0: mb.UpdateAndNotify(0, None.Deserialize(reader, mb.Path.GetNested(0)), shouldNotify)
	case 1: mb.UpdateAndNotify(1, Some.Deserialize(reader, mb.Path.GetNested(1)), shouldNotify)
	default: reader.SkipToEnd()
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
			mb.onUpdate.Invoke(mb, args)
		}
	}

	mb.tag = newTag
	mb.variant = newVariant
}

type None struct {
	path *pathpkg.Path
}

func NewNone(path *pathpkg.Path) None {
	none := None{}

	if path != nil {
		none.Path = path
	} else {
		none.Path = pathpkg.Root
	}

	return none
}

func (n *None) ClearUpdateHandlers() { }

func (n *None) Deserialize(reader readerpkg.IReader, path *pathpkg.Path) None {
	none := NewNone(path)
	none.Replace(reader, false)
	return none
}

func (n *None) GetWireType(tag uint32) *WireType {
	return nil
}

func (n *None) GetNested(tag uint32) *state.IState {
	return nil
}

func (n *None) ReplaceAt(tag uint32, wireType WireType, reader readerpkg.IReader, shouldNotify bool) {
	reader.SkipField(wireType)
}

func (n *None) ReplayListPush(reader readerpkg.IReader) { panic("not supported") }
func (n *None) ReplayListPop(reader readerpkg.IReader) { panic("not supported") }
func (n *None) ReplayMapRemove(reader readerpkg.IReader) { panic("not supported") }

func (n *None) MaybeNotify(
	tag uint32,
	newValue interface{},
	oldValue interface{},
	handler *EventHandler,
	shouldNotify bool,
) interface{} {
	if shouldNotify {
		var args = NewFieldUpdateEventArgs(tag, newValue, oldValue, n)
		if handler != nil {
			handler.Invoke(n, args)
		}
	}

	return newValue
}

type Some struct {
	path       *pathpkg.Path
	f0         interface{}
	onF0Update *EventHandler
}

func NewSome(path *pathpkg.Path) Some {
	some := Some{}

	if path != nil {
		some.Path = path
	} else {
		some.Path = pathpkg.Root
	}

	some.f0 = StateFactory.Construct(some.path.GetNested(0))

	return some
}

func (s *Some) ClearF0UpdateHandlers() {
	s.onF0Update = nil
}

func (s *Some) ClearUpdateHandlers() {
	s.onF0Update = nil
}

func (s *Some) Deserialize(reader readerpkg.IReader, path *pathpkg.Path) Some {
	some := NewSome(path)
	some.Replace(reader, false)
	return some
}

func (s *Some) GetWireType(tag uint32) *WireType {
	switch tag {
	case 0:
		if StateFactory.IsStateType(reflect.TypeOf(s.f0)) {
			return &WireTypeSized
		} else {
			return &WireTypeVarint
		}
	default:
		return nil
	}
}

func (s *Some) GetNested(tag uint32) *state.IState {
	switch tag {
	case 0:
		return &s.f0
	default:
		return nil
	}
}

func (s *Some) ReplaceAt(tag uint32, wireType WireType, reader readerpkg.IReader, shouldNotify bool) {
	switch tag {
	case 0:
		s.f0 = s.MaybeNotify(0, StateFactory.Deserialize(reader, s.path, 0), s.f0, s.onF0Update, shouldNotify)
	default:
		reader.SkipField(wireType)
	}
}

func (s *Some) ReplayListPush(reader readerpkg.IReader)  { panic("not supported") }
func (s *Some) ReplayListPop(reader readerpkg.IReader)   { panic("not supported") }
func (s *Some) ReplayMapRemove(reader readerpkg.IReader) { panic("not supported") }

func (s *Some) MaybeNotify(
	tag uint32,
	newValue interface{},
	oldValue interface{},
	handler *EventHandler,
	shouldNotify bool,
) interface{} {
	if shouldNotify {
		var args = NewFieldUpdateEventArgs(tag, newValue, oldValue, s)
		if handler != nil {
			handler.Invoke(s, args)
		}
	}

	return newValue
}
