package origin

import "github.com/axieinfinity/steit-go/pkg/path"

type ActionMessage struct {
	Path          *path.Path
	Index         uint32
	Actions       collections.Vector
	CardPlayHints collections.Vector
	Timestamp     int64
	TimeToTurnEnd int32
}

func NewActionMessage(p *path.Path, tag uint32) ActionMessage {
	obj := ActionMessage{}

	if p != nil {
		obj.Path = p
	} else {
		obj.Path = path.Root
	}
	obj.Actions = collections.NewVector(obj.Path.GetNested(1), 0)
	obj.CardPlayHints = collections.NewVector(obj.Path.GetNested(2), 0)
	return obj
}

func (s *ActionMessage) ClearIndexUpdateHandlers()         { s.OnIndexUpdate = nil }
func (s *ActionMessage) ClearActionsUpdateHandlers()       { s.OnActionsUpdate = nil }
func (s *ActionMessage) ClearCardPlayHintsUpdateHandlers() { s.OnCardPlayHintsUpdate = nil }
func (s *ActionMessage) ClearTimestampUpdateHandlers()     { s.OnTimestampUpdate = nil }
func (s *ActionMessage) ClearTimeToTurnEndUpdateHandlers() { s.OnTimeToTurnEndUpdate = nil }

func (s *ActionMessage) ClearUpdateHandlers() {
	s.OnIndexUpdate = nil
	s.OnActionsUpdate = nil
	s.OnCardPlayHintsUpdate = nil
	s.OnTimestampUpdate = nil
	s.OnTimeToTurnEndUpdate = nil
}

func (s *ActionMessage) Deserialize(reader IReader, path *Path) ActionMessage {
	actionMessage := NewActionMessage(path, 0)
	actionMessage.Replace(reader, false)
	return actionMessage
}

func (s *ActionMessage) GetWireType(tag uint32) *WireType {
	switch tag {
	case 0:
		return &WireType.Varint
	case 1:
		return &WireType.Sized
	case 2:
		return &WireType.Sized
	case 3:
		return &WireType.Varint
	case 4:
		return &WireType.Varint
	default:
		return nil
	}
}

func (s *ActionMessage) GetNested(tag uint32) *IState {
	switch tag {
	case 1:
		return &s.Actions
	case 2:
		return &s.CardPlayHints
	default:
		return nil
	}
}

func (s *ActionMessage) ReplaceAt(tag uint32, wireType WireType, reader IReader, shouldNotify bool) {
	switch tag {
	case 0:
		s.Index = s.MaybeNotify(0, reader.Readuint32(), s.Index, s.OnIndexUpdate, shouldNotify)
	case 1:
		s.Actions = s.MaybeNotify(1, Vector.Deserialize(reader, s.Path.GetNested(1)), s.Actions, s.OnActionsUpdate, shouldNotify)
	case 2:
		s.CardPlayHints = s.MaybeNotify(2, Vector.Deserialize(reader, s.Path.GetNested(2)), s.CardPlayHints, s.OnCardPlayHintsUpdate, shouldNotify)
	case 3:
		s.Timestamp = s.MaybeNotify(3, reader.Readint64(), s.Timestamp, s.OnTimestampUpdate, shouldNotify)
	case 4:
		s.TimeToTurnEnd = s.MaybeNotify(4, reader.Readint32(), s.TimeToTurnEnd, s.OnTimeToTurnEndUpdate, shouldNotify)
	default:
		reader.SkipField(wireType)
	}
}

func (s *ActionMessage) ReplayListPush(reader IReader) { panic("") }
func (s *ActionMessage) ReplayListPop()                { panic("") }
func (s *ActionMessage) ReplayMapRemove(key uint32)    { panic("") }

func (s *ActionMessage) MaybeNotify(
	tag uint32,
	newValue TValue,
	oldValue TValue,
	handler EventHandler,
	shouldNotify bool,
) TValue {
	if shouldNotify {
		args := FieldUpdateEventArgs(tag, newValue, oldValue, s)
		handler.Invoke(s, args)
	}

	return newValue
}
