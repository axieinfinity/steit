package origin

import (
	"reflect"

	"github.com/axieinfinity/steit-go/pkg/path"
)

type BinaryMessageEnum uint32

const (
	BinaryMessageActionMessageTag  BinaryMessageEnum = 0
	BinaryMessageCaptchaMessageTag BinaryMessageEnum = 1
)

type BinaryMessage struct {
	Path *path.Path

	Tag      uint32
	Variant  IState
	OnUpdate *EventHandler
}

func (s *BinaryMessage) ActionMessageVariant() BinaryMessageActionMessage {
	return s.Variant
}
func (s *BinaryMessage) CaptchaMessageVariant() BinaryMessageCaptchaMessage {
	return s.Variant
}
func NewBinaryMessage(path *Path, tag uint32) BinaryMessage {
	obj := BinaryMessage{Tag: tag}
	if path != nil {
		obj.Path = path
	} else {
		obj.Path = Path.Root
	}
	switch tag {
	case 0:
		obj.Variant = NewBinaryMessageActionMessage(obj.Path.GetNested(0), 0)
	case 1:
		obj.Variant = NewBinaryMessageCaptchaMessage(obj.Path.GetNested(1), 0)
	default:
		obj.Variant = NewBinaryMessageActionMessage(obj.Path.GetNested(0), 0)
	}
	return obj
}

func (s *BinaryMessage) NewActionMessage(path *Path, tag uint32) BinaryMessage {
	return NewBinaryMessage(path, 0)
}
func (s *BinaryMessage) NewCaptchaMessage(path *Path, tag uint32) BinaryMessage {
	return NewBinaryMessage(path, 1)
}

func (s *BinaryMessage) ClearUpdateHandlers() {
	s.OnUpdate = nil
}

func BinaryMessageDeserialize(_type reflect.Type, reader IReader, path *Path) BinaryMessage {
	binaryMessage := NewBinaryMessage(path, 0)
	binaryMessage.Replace(reader, false)
	return binaryMessage
}

func (s *BinaryMessage) GetWireType(tag uint32) *WireType {
	switch tag {
	case 0:
		return &WireType.Sized
	case 1:
		return &WireType.Sized
	default:
		return nil
	}
}

func (s *BinaryMessage) GetNested(tag uint32) *IState {
	if tag == s.Tag {
		return &s.Variant
	} else {
		return nil
	}
}

func (s *BinaryMessage) ReplaceAt(tag uint32, wireType WireType, reader IReader, shouldNotify bool) {
	switch tag {
	case 0:
		s.UpdateAndNotify(0, BinaryMessageActionMessageDeserialize(reader, s.Path.GetNested(0)), shouldNotify)
	case 1:
		s.UpdateAndNotify(1, BinaryMessageCaptchaMessageDeserialize(reader, s.Path.GetNested(1)), shouldNotify)
	default:
		reader.SkipToEnd()
	}
}

func (s *BinaryMessage) ReplayListPush(IReader reader)  { panic("") }
func (s *BinaryMessage) ReplayListPop(IReader reader)   { panic("") }
func (s *BinaryMessage) ReplayMapRemove(IReader reader) { panic("") }

func (s *BinaryMessage) UpdateAndNotify(newTag uint32, newVariant IState, shouldNotify bool) {
	if shouldNotify {
		args := NewVariantUpdateEventArgs(newTag, newVariant, s.Tag, s.Variant, s)
		BinaryMessage.OnUpdate.Invoke(this, args)
	}

	s.Tag = newTag
	s.Variant = newVariant
}

// Variant (0): ActionMessage
type BinaryMessageActionMessage struct {
	Path            *Path
	OnMessageUpdate *EventHandler
	Message         ActionMessage
}

func NewBinaryMessageActionMessage(path *Path, tag uint32) BinaryMessageActionMessage {
	obj := BinaryMessageActionMessage{}

	if path != nil {
		obj.Path = path
	} else {
		obj.Path = Path.Root
	}
	obj.Message = NewActionMessage(obj.Path.GetNested(0), 0)
	return obj
}

func (s *BinaryMessageActionMessage) ClearMessageUpdateHandlers() {
	s.OnMessageUpdate = nil
}

func (s *BinaryMessageActionMessage) ClearUpdateHandlers() {
	s.OnMessageUpdate = nil
}

func BinaryMessageActionMessageDeserialize(_type reflect.Type, reader IReader, path *Path) BinaryMessageActionMessage {
	actionMessage := NewBinaryMessageActionMessage(path, 0)
	actionMessage.Replace(reader, false)
	return actionMessage
}

func (s *BinaryMessageActionMessage) GetWireType(tag uint32) *WireType {
	switch tag {
	case 0:
		return &WireType.Sized
	default:
		return nil
	}
}

func (s *BinaryMessageActionMessage) GetNested(tag uint32) *IState {
	switch tag {
	case 0:
		return &s.Message
	default:
		return nil
	}
}

func (s *BinaryMessageActionMessage) ReplaceAt(tag uint32, wireType WireType, reader IReader, shouldNotify bool) {
	switch tag {
	case 0:
		s.Message = s.MaybeNotify(0, ActionMessageDeserialize(reader, s.Path.GetNested(0)), s.Message, s.OnMessageUpdate, shouldNotify)
	default:
		reader.SkipField(wireType)
	}
}

func (s *BinaryMessageActionMessage) ReplayListPush(reader IReader) { panic("") }
func (s *BinaryMessageActionMessage) ReplayListPop()                { panic("") }
func (s *BinaryMessageActionMessage) ReplayMapRemove(key uint32)    { panic("") }

func (s *BinaryMessageActionMessage) MaybeNotify(
	tag uint32,
	newValue interface{},
	oldValue interface{},
	handler EventHandler,
	shouldNotify bool,
) interface{} {
	if shouldNotify {
		args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
		handler.Invoke(s, args)
	}

	return newValue
}

// Variant (1): CaptchaMessage
type BinaryMessageCaptchaMessage struct {
	Path            *Path
	OnMessageUpdate *EventHandler
	Message         CaptchaMessage
}

func NewBinaryMessageCaptchaMessage(path *Path, tag uint32) BinaryMessageCaptchaMessage {
	obj := BinaryMessageCaptchaMessage{}

	if path != nil {
		obj.Path = path
	} else {
		obj.Path = Path.Root
	}
	obj.Message = NewCaptchaMessage(obj.Path.GetNested(0), 0)
	return obj
}

func (s *BinaryMessageCaptchaMessage) ClearMessageUpdateHandlers() {
	s.OnMessageUpdate = nil
}

func (s *BinaryMessageCaptchaMessage) ClearUpdateHandlers() {
	s.OnMessageUpdate = nil
}

func BinaryMessageCaptchaMessageDeserialize(_type reflect.Type, reader IReader, path *Path) BinaryMessageCaptchaMessage {
	captchaMessage := NewBinaryMessageCaptchaMessage(path, 0)
	captchaMessage.Replace(reader, false)
	return captchaMessage
}

func (s *BinaryMessageCaptchaMessage) GetWireType(tag uint32) *WireType {
	switch tag {
	case 0:
		return &WireType.Sized
	default:
		return nil
	}
}

func (s *BinaryMessageCaptchaMessage) GetNested(tag uint32) *IState {
	switch tag {
	case 0:
		return &s.Message
	default:
		return nil
	}
}

func (s *BinaryMessageCaptchaMessage) ReplaceAt(tag uint32, wireType WireType, reader IReader, shouldNotify bool) {
	switch tag {
	case 0:
		s.Message = s.MaybeNotify(0, CaptchaMessageDeserialize(reader, s.Path.GetNested(0)), s.Message, s.OnMessageUpdate, shouldNotify)
	default:
		reader.SkipField(wireType)
	}
}

func (s *BinaryMessageCaptchaMessage) ReplayListPush(reader IReader) { panic("") }
func (s *BinaryMessageCaptchaMessage) ReplayListPop()                { panic("") }
func (s *BinaryMessageCaptchaMessage) ReplayMapRemove(key uint32)    { panic("") }

func (s *BinaryMessageCaptchaMessage) MaybeNotify(
	tag uint32,
	newValue interface{},
	oldValue interface{},
	handler EventHandler,
	shouldNotify bool,
) interface{} {
	if shouldNotify {
		args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
		handler.Invoke(s, args)
	}

	return newValue
}
