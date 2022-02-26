package origin

import (
	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
	statepkg "github.com/axieinfinity/steit-go/pkg/state"
)

type BinaryMessageEnum uint32

const (
	BinaryMessageActionMessageTag  BinaryMessageEnum = 0
	BinaryMessageCaptchaMessageTag BinaryMessageEnum = 1
)

type BinaryMessage struct {
	Path *path.Path

	Tag     uint32
	Variant statepkg.IState
}

func (s *BinaryMessage) ActionMessageVariant() *BinaryMessageActionMessage {
	v, ok := s.Variant.(*BinaryMessageActionMessage)
	if !ok {
		return nil
	}
	return v
}

// func (s *BinaryMessage) CaptchaMessageVariant() BinaryMessageCaptchaMessage {
// 	v, ok := s.Variant.(*BinaryMessageCaptchaMessage)
// 	if !ok {
// 		return nil
// 	}
// 	return v
// }

func NewBinaryMessage(p *path.Path, tag uint32) *BinaryMessage {
	obj := &BinaryMessage{Tag: tag}
	if p != nil {
		obj.Path = p
	} else {
		obj.Path = path.Root
	}

	switch tag {
	case 0:
		obj.Variant = NewBinaryMessageActionMessage(obj.Path.GetNested(0), 0)
	// case 1:
	// 	obj.Variant = NewBinaryMessageCaptchaMessage(obj.Path.GetNested(1), 0)
	default:
		obj.Variant = NewBinaryMessageActionMessage(obj.Path.GetNested(0), 0)
	}

	return obj
}

func NewBinaryActionMessage(path *path.Path, tag uint32) *BinaryMessage {
	return NewBinaryMessage(path, 0)
}

func NewBinaryCaptchaMessage(path *path.Path, tag uint32) *BinaryMessage {
	return NewBinaryMessage(path, 1)
}

func (b *BinaryMessage) Deserialize(reader readerpkg.IReader, p *path.Path) error {
	binaryMessage := NewBinaryMessage(p, 0)
	statepkg.Replace(binaryMessage, reader, false)
	*b = *binaryMessage
	return nil
}

func (s *BinaryMessage) GetWireType(tag uint32) *codec.WireType {
	switch tag {
	case 0:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	case 1:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	default:
		return nil
	}
}

func (s *BinaryMessage) GetNested(tag uint32) statepkg.IState {
	if tag == s.Tag {
		return s.Variant
	} else {
		return nil
	}
}

func (s *BinaryMessage) GetPath() *path.Path {
	return s.Path
}

func (s *BinaryMessage) ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool) {
	switch tag {
	case 0:

		// BinaryMessageActionMessageDeserialize(reader, s.Path.GetNested(0))
		// s.UpdateAndNotify(0, s, shouldNotify)
	// case 1:
	// 	s.UpdateAndNotify(1, BinaryMessageCaptchaMessageDeserialize(reader, s.Path.GetNested(1)), shouldNotify)
	default:
		readerpkg.SkipToEnd(reader)
	}
}

func (s *BinaryMessage) ReplayListPush(reader readerpkg.IReader) { panic("") }
func (s *BinaryMessage) ReplayListPop()                          { panic("") }
func (s *BinaryMessage) ReplayMapRemove(uint32)                  { panic("") }

func (s *BinaryMessage) UpdateAndNotify(newTag uint32, newVariant statepkg.IState, shouldNotify bool) {
	s.Tag = newTag
	s.Variant = newVariant
}

// Variant (0): ActionMessage
type BinaryMessageActionMessage struct {
	Path    *path.Path
	Message *ActionMessage
}

func NewBinaryMessageActionMessage(p *path.Path, tag uint32) *BinaryMessageActionMessage {
	obj := &BinaryMessageActionMessage{}

	if p != nil {
		obj.Path = p
	} else {
		obj.Path = path.Root
	}
	obj.Message = NewActionMessage(obj.Path.GetNested(0), 0)
	return obj
}

func (s *BinaryMessageActionMessage) Deserialize(reader readerpkg.IReader, path *path.Path) error {
	actionMessage := NewBinaryMessageActionMessage(path, 0)
	statepkg.Replace(actionMessage, reader, false)
	*s = *actionMessage
	return nil
}

func (s *BinaryMessageActionMessage) GetWireType(tag uint32) *codec.WireType {
	switch tag {
	case 0:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	default:
		return nil
	}
}

func (s *BinaryMessageActionMessage) GetPath() *path.Path {
	return s.Path
}

func (s *BinaryMessageActionMessage) GetNested(tag uint32) statepkg.IState {
	switch tag {
	case 0:
		return s.Message
	default:
		return nil
	}
}

func (s *BinaryMessageActionMessage) ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool) {
	switch tag {
	case 0:
		s.Message.Deserialize(reader, s.Path.GetNested(0))
	default:
		readerpkg.SkipField(reader, wireType)
	}
}

func (s *BinaryMessageActionMessage) ReplayListPush(reader readerpkg.IReader) { panic("") }
func (s *BinaryMessageActionMessage) ReplayListPop()                          { panic("") }
func (s *BinaryMessageActionMessage) ReplayMapRemove(key uint32)              { panic("") }

// // Variant (1): CaptchaMessage
// type BinaryMessageCaptchaMessage struct {
// 	Path            *Path
// 	OnMessageUpdate *EventHandler
// 	Message         CaptchaMessage
// }

// func NewBinaryMessageCaptchaMessage(path *Path, tag uint32) BinaryMessageCaptchaMessage {
// 	obj := BinaryMessageCaptchaMessage{}

// 	if path != nil {
// 		obj.Path = path
// 	} else {
// 		obj.Path = Path.Root
// 	}
// 	obj.Message = NewCaptchaMessage(obj.Path.GetNested(0), 0)
// 	return obj
// }

// func (s *BinaryMessageCaptchaMessage) ClearMessageUpdateHandlers() {
// 	s.OnMessageUpdate = nil
// }

// func (s *BinaryMessageCaptchaMessage) ClearUpdateHandlers() {
// 	s.OnMessageUpdate = nil
// }

// func BinaryMessageCaptchaMessageDeserialize(_type reflect.Type, reader IReader, path *Path) BinaryMessageCaptchaMessage {
// 	captchaMessage := NewBinaryMessageCaptchaMessage(path, 0)
// 	captchaMessage.Replace(reader, false)
// 	return captchaMessage
// }

// func (s *BinaryMessageCaptchaMessage) GetWireType(tag uint32) *WireType {
// 	switch tag {
// 	case 0:
// 		return &WireType.Sized
// 	default:
// 		return nil
// 	}
// }

// func (s *BinaryMessageCaptchaMessage) GetNested(tag uint32) *IState {
// 	switch tag {
// 	case 0:
// 		return &s.Message
// 	default:
// 		return nil
// 	}
// }

// func (s *BinaryMessageCaptchaMessage) ReplaceAt(tag uint32, wireType WireType, reader readerpkg.IReader, shouldNotify bool) {
// 	switch tag {
// 	case 0:
// 		s.Message = s.MaybeNotify(0, CaptchaMessageDeserialize(reader, s.Path.GetNested(0)), s.Message, s.OnMessageUpdate, shouldNotify)
// 	default:
// 		reader.SkipField(wireType)
// 	}
// }

// func (s *BinaryMessageCaptchaMessage) ReplayListPush(reader readerpkg.IReader) { panic("") }
// func (s *BinaryMessageCaptchaMessage) ReplayListPop()                          { panic("") }
// func (s *BinaryMessageCaptchaMessage) ReplayMapRemove(key uint32)              { panic("") }
