package origin

import (
	"github.com/axieinfinity/steit-go/pkg/builtin/option"
	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/collections/vector/vec_u32"
	"github.com/axieinfinity/steit-go/pkg/event"
	"github.com/axieinfinity/steit-go/pkg/path"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
	statepkg "github.com/axieinfinity/steit-go/pkg/state"
)

var _ statepkg.IState = (*CardPlayHint)(nil)

type CardPlayHint struct {
	Path                  *path.Path
	CardId                uint32
	AreTargetsPredictable bool
	TargetCandidates      *vec_u32.VecU32
	NumOptions            *option.OptionUint32
	NumTargets            *option.OptionUint32
	CardCandidates        *vec_u32.VecU32
	NumCards              *option.OptionUint32
	MinNumCards           *option.OptionUint32
	PositionCandidates    *vec_u32.VecU32
	NumPositions          *option.OptionUint32
}

func NewCardPlayHint(p *path.Path) *CardPlayHint {
	obj := &CardPlayHint{}

	if p != nil {
		obj.Path = p
	} else {
		obj.Path = path.Root
	}

	obj.TargetCandidates = vec_u32.NewVecU32(obj.Path.GetNested(2), nil)
	obj.NumOptions = option.NewOptionUint32(obj.Path.GetNested(3))
	obj.NumTargets = option.NewOptionUint32(obj.Path.GetNested(4))
	obj.CardCandidates = vec_u32.NewVecU32(obj.Path.GetNested(5), nil)
	obj.NumCards = option.NewOptionUint32(obj.Path.GetNested(6))
	obj.MinNumCards = option.NewOptionUint32(obj.Path.GetNested(7))
	obj.PositionCandidates = vec_u32.NewVecU32(obj.Path.GetNested(8), nil)
	obj.NumPositions = option.NewOptionUint32(obj.Path.GetNested(9))
	return obj
}

func (s *CardPlayHint) Deserialize(reader readerpkg.IReader, path *path.Path) error {
	cardPlayHint := NewCardPlayHint(path)
	statepkg.Replace(cardPlayHint, reader, false)
	*s = *cardPlayHint
	return nil
}

func (s *CardPlayHint) GetWireType(tag uint32) *codec.WireType {
	switch tag {
	case 0:
		return codec.GetWireTypePtr(codec.WireTypeVarint)
	case 1:
		return codec.GetWireTypePtr(codec.WireTypeVarint)
	case 2:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	case 3:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	case 4:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	case 5:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	case 6:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	case 7:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	case 8:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	case 9:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	default:
		return nil
	}
}

func (s *CardPlayHint) GetNested(tag uint32) statepkg.IState {
	switch tag {
	case 2:
		return s.TargetCandidates
	case 3:
		return s.NumOptions
	case 4:
		return s.NumTargets
	case 5:
		return s.CardCandidates
	case 6:
		return s.NumCards
	case 7:
		return s.MinNumCards
	case 8:
		return s.PositionCandidates
	case 9:
		return s.NumPositions
	default:
		return nil
	}
}

func (s *CardPlayHint) ReplaceAt(tag uint32, wireType codec.WireType, reader readerpkg.IReader, shouldNotify bool) {
	switch tag {
	case 0:
		s.CardId = readerpkg.ReadUInt32(reader)
	case 1:
		s.AreTargetsPredictable = readerpkg.ReadBoolean(reader)
	case 2:
		s.TargetCandidates.Deserialize(reader, s.Path.GetNested(2))
	case 3:
		s.NumOptions.Deserialize(reader, s.Path.GetNested(3))
	case 4:
		s.NumTargets.Deserialize(reader, s.Path.GetNested(4))
	case 5:
		s.CardCandidates.Deserialize(reader, s.Path.GetNested(5))
	case 6:
		s.NumCards.Deserialize(reader, s.Path.GetNested(6))
	case 7:
		s.MinNumCards.Deserialize(reader, s.Path.GetNested(7))
	case 8:
		s.PositionCandidates.Deserialize(reader, s.Path.GetNested(8))
	case 9:
		s.NumPositions.Deserialize(reader, s.Path.GetNested(9))
	default:
		readerpkg.SkipField(reader, wireType)
	}
}

func (s *CardPlayHint) ReplayListPush(reader readerpkg.IReader) { panic("") }
func (s *CardPlayHint) ReplayListPop()                          { panic("") }
func (s *CardPlayHint) ReplayMapRemove(key uint32)              { panic("") }

func (s *CardPlayHint) MaybeNotify(
	tag uint32,
	newValue interface{},
	oldValue interface{},
	handler event.EventHandler,
	shouldNotify bool,
) interface{} {
	return newValue
}

func (s *CardPlayHint) GetPath() *path.Path {
	return s.Path
}
