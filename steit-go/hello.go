package main

import (
	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/collections/vector/vec_i32"
	"github.com/axieinfinity/steit-go/pkg/path"
	"github.com/axieinfinity/steit-go/pkg/reader"
	readerpkg "github.com/axieinfinity/steit-go/pkg/reader"
	statepkg "github.com/axieinfinity/steit-go/pkg/state"
)

type Hello struct {
	Path    *path.Path
	Numbers *vec_i32.VecI32
	Others  *vec_i32.VecI32
}

func NewHello(p *path.Path, tag uint32) *Hello {
	obj := &Hello{}

	if p != nil {
		obj.Path = p
	} else {
		obj.Path = path.Root
	}

	obj.Numbers = vec_i32.NewVec(obj.Path.GetNested(0), nil)
	obj.Others = vec_i32.NewVec(obj.Path.GetNested(1), nil)
	return obj
}

func (s *Hello) Deserialize(reader reader.IReader, path *path.Path) error {
	actionMessage := NewHello(path, 0)
	statepkg.Replace(actionMessage, reader, false)
	*s = *actionMessage
	return nil
}

func (s *Hello) GetPath() *path.Path {
	return s.Path
}

func (s *Hello) GetWireType(tag uint32) *codec.WireType {
	switch tag {
	case 0:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	case 1:
		return codec.GetWireTypePtr(codec.WireTypeSized)
	default:
		return nil
	}
}

func (s *Hello) GetNested(tag uint32) statepkg.IState {
	switch tag {
	case 0:
		return s.Numbers
	case 1:
		return s.Others
	default:
		return nil
	}
}

func (s *Hello) ReplaceAt(tag uint32, wireType codec.WireType, reader reader.IReader, shouldNotify bool) {
	switch tag {
	case 0:
		s.Numbers.Deserialize(reader, s.Path.GetNested(0))
	case 1:
		s.Others.Deserialize(reader, s.Path.GetNested(1))
	default:
		readerpkg.SkipField(reader, wireType)
	}
}

func (s *Hello) ReplayListPush(reader reader.IReader) { panic("") }
func (s *Hello) ReplayListPop()                       { panic("") }
func (s *Hello) ReplayMapRemove(key uint32)           { panic("") }
