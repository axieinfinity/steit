package bytes

import (
	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/path"
	"github.com/axieinfinity/steit-go/pkg/reader"
	"github.com/axieinfinity/steit-go/pkg/state"
)

var _ state.IState = (*Bytes)(nil)

type Bytes struct {
	byteValues []byte
	path       *path.Path
}

func (b *Bytes) GetPath() *path.Path {
	return b.path
}

func NewBytes(p *path.Path, data []byte) *Bytes {
	newPath := p
	if p == nil {
		newPath = path.Root
	}
	if data == nil {
		data = []byte{}
	}
	return &Bytes{path: newPath, byteValues: data}
}

func (b *Bytes) Deserialize(r reader.IReader, p *path.Path) error {
	*b = Bytes{
		path:       p,
		byteValues: reader.ReadToEnd(r),
	}
	return nil
}

func (s *Bytes) GetWireType(uint32) *codec.WireType {
	return nil
}

func (s *Bytes) GetNested(uint32) state.IState {
	return nil
}

func (s *Bytes) ReplaceAt(tag uint32, wireType codec.WireType, reader reader.IReader, shouldNotify bool) {
	panic("not supported")
}

func (s *Bytes) ReplayListPush(reader reader.IReader) {
	panic("not supported")
}

func (s *Bytes) ReplayListPop() {
	panic("not supported")
}

func (s *Bytes) ReplayMapRemove(uint32) {
	panic("not supported")
}

// // public static Bytes Deserialize(IReader reader, Path? path = null) {
// 	public static Bytes Deserialize(IReader reader, Path path = null) {
// 		return new Bytes(path, reader.ReadToEnd());
// 	}
