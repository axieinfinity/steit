package option

import (
	"fmt"
	"reflect"

	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/path"
	"github.com/axieinfinity/steit-go/pkg/reader"
	"github.com/axieinfinity/steit-go/pkg/state"
)

var _ state.IState = (*Option)(nil)

type Option struct {
	path           *path.Path
	isSome         bool
	valueOrDefault interface{}
	_type          reflect.Type
}

type optionOpts struct {
	value interface{}
}

type OptionOptArgs func(*optionOpts)

func (o *Option) GetValueOrDefault() interface{} {
	return o.valueOrDefault
}

func (o *Option) IsSome() bool {
	return o.isSome
}

func (o *Option) IsNone() bool {
	return !o.isSome
}

func NewOption(p *path.Path, opts ...OptionOptArgs) *Option {
	var res *Option
	if p == nil {
		res = &Option{path: path.Root}
	} else {
		res = &Option{path: p}
	}

	oo := &optionOpts{}
	for _, opt := range opts {
		opt(oo)
	}

	if oo.value != nil {
		res.isSome = true
		res.valueOrDefault = oo.value
		res._type = reflect.TypeOf(oo.value)
	}

	return res
}

func Some(p *path.Path, value interface{}) *Option {
	return NewOption(p, WithValue(value))
}

func None(p *path.Path) *Option {
	return NewOption(p)
}

func WithValue(value interface{}) OptionOptArgs {
	return func(oo *optionOpts) {
		oo.value = value
	}
}

func Deserialize(_type reflect.Type, r reader.IReader, p *path.Path) *Option {
	if !r.EndOfStream() {
		return Some(p, state.DeserializeNested(_type, r, p, 0))
	} else {
		return None(p)
	}
}

func (o *Option) GetPath() *path.Path {
	return o.path
}

func (s *Option) GetWireType(uint32) *codec.WireType {
	return nil
}

func (s *Option) GetNested(uint32) state.IState {
	return nil
}

func (s *Option) ReplaceAt(tag uint32, wireType codec.WireType, reader reader.IReader, shouldNotify bool) {
	panic("not supported")
}

func (s *Option) ReplayListPush(reader reader.IReader) {
	panic("not supported")
}

func (s *Option) ReplayListPop() {
	panic("not supported")
}

func (s *Option) ReplayMapRemove(uint32) {
	panic("not supported")
}

func (o *Option) String() string {
	if o.IsSome() {
		return fmt.Sprintf("Some(%v)", o.valueOrDefault)
	}
	return "None"
}
