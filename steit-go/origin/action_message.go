package origin

import (
	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/path"
	"github.com/axieinfinity/steit-go/pkg/reader"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
	statepkg "github.com/axieinfinity/steit-go/pkg/state"
)

type ActionMessage struct {
	Path  *path.Path
	Index uint32
	// Actions       collections.Vector
	CardPlayHints *VecCardPlayHint
	Timestamp     int64
	TimeToTurnEnd int32
}

func NewActionMessage(p *path.Path, tag uint32) *ActionMessage {
	obj := &ActionMessage{}

	if p != nil {
		obj.Path = p
	} else {
		obj.Path = path.Root
	}
	// obj.Actions = collections.NewVector(obj.Path.GetNested(1), 0)
	obj.CardPlayHints = NewVecCardPlayHint(obj.Path.GetNested(2), nil)
	return obj
}

func (s *ActionMessage) Deserialize(reader reader.IReader, path *path.Path) error {
	actionMessage := NewActionMessage(path, 0)
	statepkg.Replace(actionMessage, reader, false)
	*s = *actionMessage
	return nil
}

func (s *ActionMessage) GetPath() *path.Path {
	return s.Path
}

func (s *ActionMessage) GetWireType(tag uint32) *codec.WireType {
	switch tag {
	case 0:
		return codec.GetWireTypePtr(codec.WireTypeVarint)
	case 1:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	case 2:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	case 3:
		return codec.GetWireTypePtr(codec.WireTypeVarint)
	case 4:
		return codec.GetWireTypePtr(codec.WireTypeVarint)
	default:
		return nil
	}
}

func (s *ActionMessage) GetNested(tag uint32) statepkg.IState {
	switch tag {
	case 1:
		return nil
	case 2:
		return s.CardPlayHints
	default:
		return nil
	}
}

func (s *ActionMessage) ReplaceAt(tag uint32, wireType codec.WireType, reader reader.IReader, shouldNotify bool) {
	switch tag {
	case 0:
		s.Index = readerpkg.ReadUInt32(reader)
	case 1:
	// s.Actions = s.MaybeNotify(1, Vector.Deserialize(reader, s.Path.GetNested(1)), s.Actions, s.OnActionsUpdate, shouldNotify)
	case 2:
		s.CardPlayHints.Deserialize(reader, s.Path.GetNested(2))
	case 3:
		s.Timestamp = readerpkg.ReadInt64(reader)
	case 4:
		s.TimeToTurnEnd = readerpkg.ReadInt32(reader)
	default:
		readerpkg.SkipField(reader, wireType)
	}
}

func (s *ActionMessage) ReplayListPush(reader reader.IReader) { panic("") }
func (s *ActionMessage) ReplayListPop()                       { panic("") }
func (s *ActionMessage) ReplayMapRemove(key uint32)           { panic("") }
