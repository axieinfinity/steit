package origin

import (
	"github.com/axieinfinity/steit-go/pkg/builtin/option"
	"github.com/axieinfinity/steit-go/pkg/collections"
	"github.com/axieinfinity/steit-go/pkg/collections/vector"
	"github.com/axieinfinity/steit-go/pkg/path"
	statepkg "github.com/axieinfinity/steit-go/pkg/state"
)

type CardPlayHint struct {
	Path                  *path.Path
	CardId                uint32
	AreTargetsPredictable bool
	TargetCandidates      *collections.Vector
	NumOptions            *option.Option
	NumTargets            *option.Option
	CardCandidates        *collections.Vector
	NumCards              *option.Option
	MinNumCards           *option.Option
	PositionCandidates    *collections.Vector
	NumPositions          *option.Option
}

func NewCardPlayHint(p *path.Path, tag uint32) CardPlayHint {
	obj := CardPlayHint{}

	if p != nil {
		obj.Path = p
	} else {
		obj.Path = path.Root
	}
	targetCandidates := []uint32{}
	obj.TargetCandidates = vector.NewVector(obj.Path.GetNested(2), targetCandidates)
	obj.NumOptions = option.NewOption(obj.Path.GetNested(3), 0)
	obj.NumTargets = option.NewOption(obj.Path.GetNested(4), 0)
	obj.CardCandidates = vector.NewVector(obj.Path.GetNested(5), 0)
	obj.NumCards = option.NewOption(obj.Path.GetNested(6), 0)
	obj.MinNumCards = option.NewOption(obj.Path.GetNested(7), 0)
	obj.PositionCandidates = vector.NewVector(obj.Path.GetNested(8), 0)
	obj.NumPositions = option.NewOption(obj.Path.GetNested(9), 0)
	return obj
}

func (s *CardPlayHint) Deserialize(reader IReader, path *Path) CardPlayHint {
	cardPlayHint := NewCardPlayHint(path, 0)
	cardPlayHint.Replace(reader, false)
	return cardPlayHint
}

func (s *CardPlayHint) GetWireType(tag uint32) *WireType {
	switch tag {
	case 0:
		return &WireType.Varint
	case 1:
		return &WireType.Varint
	case 2:
		return &WireType.Sized
	case 3:
		return &WireType.Sized
	case 4:
		return &WireType.Sized
	case 5:
		return &WireType.Sized
	case 6:
		return &WireType.Sized
	case 7:
		return &WireType.Sized
	case 8:
		return &WireType.Sized
	case 9:
		return &WireType.Sized
	default:
		return nil
	}
}

func (s *CardPlayHint) GetNested(tag uint32) statepkg.IState {
	switch tag {
	case 2:
		return &s.TargetCandidates
	case 3:
		return &s.NumOptions
	case 4:
		return &s.NumTargets
	case 5:
		return &s.CardCandidates
	case 6:
		return &s.NumCards
	case 7:
		return &s.MinNumCards
	case 8:
		return &s.PositionCandidates
	case 9:
		return &s.NumPositions
	default:
		return nil
	}
}

func (s *CardPlayHint) ReplaceAt(tag uint32, wireType WireType, reader IReader, shouldNotify bool) {
	switch tag {
	case 0:
		s.CardId = s.MaybeNotify(0, reader.Readuint32(), s.CardId, s.OnCardIdUpdate, shouldNotify)
	case 1:
		s.AreTargetsPredictable = s.MaybeNotify(1, reader.Readbool(), s.AreTargetsPredictable, s.OnAreTargetsPredictableUpdate, shouldNotify)
	case 2:
		s.TargetCandidates = s.MaybeNotify(2, Vector.Deserialize(reader, s.Path.GetNested(2)), s.TargetCandidates, s.OnTargetCandidatesUpdate, shouldNotify)
	case 3:
		s.NumOptions = s.MaybeNotify(3, Option.Deserialize(reader, s.Path.GetNested(3)), s.NumOptions, s.OnNumOptionsUpdate, shouldNotify)
	case 4:
		s.NumTargets = s.MaybeNotify(4, Option.Deserialize(reader, s.Path.GetNested(4)), s.NumTargets, s.OnNumTargetsUpdate, shouldNotify)
	case 5:
		s.CardCandidates = s.MaybeNotify(5, Vector.Deserialize(reader, s.Path.GetNested(5)), s.CardCandidates, s.OnCardCandidatesUpdate, shouldNotify)
	case 6:
		s.NumCards = s.MaybeNotify(6, Option.Deserialize(reader, s.Path.GetNested(6)), s.NumCards, s.OnNumCardsUpdate, shouldNotify)
	case 7:
		s.MinNumCards = s.MaybeNotify(7, Option.Deserialize(reader, s.Path.GetNested(7)), s.MinNumCards, s.OnMinNumCardsUpdate, shouldNotify)
	case 8:
		s.PositionCandidates = s.MaybeNotify(8, Vector.Deserialize(reader, s.Path.GetNested(8)), s.PositionCandidates, s.OnPositionCandidatesUpdate, shouldNotify)
	case 9:
		s.NumPositions = s.MaybeNotify(9, Option.Deserialize(reader, s.Path.GetNested(9)), s.NumPositions, s.OnNumPositionsUpdate, shouldNotify)
	default:
		reader.SkipField(wireType)
	}
}

func (s *CardPlayHint) ReplayListPush(reader IReader) { panic("") }
func (s *CardPlayHint) ReplayListPop()                { panic("") }
func (s *CardPlayHint) ReplayMapRemove(key uint32)    { panic("") }

func (s *CardPlayHint) MaybeNotify(
	tag uint32,
	newValue u,
	oldValue TValue,
	handler EventHandler,
	shouldNotify bool,
) TValue {

	return newValue
}
