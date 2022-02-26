package option

import (
	"fmt"

	"github.com/axieinfinity/steit/steit-go/pkg/builtin/primitive"
	"github.com/axieinfinity/steit/steit-go/pkg/codec"
	"github.com/axieinfinity/steit/steit-go/pkg/path"
	"github.com/axieinfinity/steit/steit-go/pkg/reader"
	"github.com/axieinfinity/steit/steit-go/pkg/state"
)

var _ state.IState = (*OptionUint32)(nil)

type OptionUint32 struct {
	path           *path.Path
	IsSomeValue    bool
	ValueOrDefault uint32
}

type optionUint32Opts struct {
	value *uint32
}

type OptionUint32OptArgs func(*optionUint32Opts)

func (o *OptionUint32) GetValueOrDefault() uint32 {
	return o.ValueOrDefault
}

func (o *OptionUint32) IsSome() bool {
	return o.IsSomeValue
}

func (o *OptionUint32) IsNone() bool {
	return !o.IsSomeValue
}

func NewOptionUint32(p *path.Path, opts ...OptionUint32OptArgs) *OptionUint32 {
	var res *OptionUint32
	if p == nil {
		res = &OptionUint32{path: path.Root}
	} else {
		res = &OptionUint32{path: p}
	}

	oo := &optionUint32Opts{}
	for _, opt := range opts {
		opt(oo)
	}

	if oo.value != nil {
		res.IsSomeValue = true
		res.ValueOrDefault = *oo.value
	}

	return res
}

func SomeUint32(p *path.Path, value uint32) *OptionUint32 {
	return NewOptionUint32(p, WithUint32Value(value))
}

func NoneUint32(p *path.Path) *OptionUint32 {
	return NewOptionUint32(p)
}

func WithUint32Value(value uint32) OptionUint32OptArgs {
	return func(oo *optionUint32Opts) {
		oo.value = &value
	}
}

func (o *OptionUint32) Deserialize(r reader.IReader, p *path.Path) error {
	if !reader.EndOfStream(r) {
		var value primitive.Uint32
		state.DeserializeNested(&value, r, p, 0)
		*o = *SomeUint32(p, uint32(value))
		return nil
	}
	*o = *NoneUint32(p)
	return nil
}

func (o *OptionUint32) GetPath() *path.Path {
	return o.path
}

func (s *OptionUint32) GetWireType(uint32) *codec.WireType {
	return nil
}

func (s *OptionUint32) GetNested(uint32) state.IState {
	return nil
}

func (s *OptionUint32) ReplaceAt(tag uint32, wireType codec.WireType, reader reader.IReader, shouldNotify bool) {
	panic("not supported")
}

func (s *OptionUint32) ReplayListPush(reader reader.IReader) {
	panic("not supported")
}

func (s *OptionUint32) ReplayListPop() {
	panic("not supported")
}

func (s *OptionUint32) ReplayMapRemove(uint32) {
	panic("not supported")
}

func (o *OptionUint32) String() string {
	if o.IsSome() {
		return fmt.Sprintf("Some(%v)", o.ValueOrDefault)
	}
	return "None"
}
