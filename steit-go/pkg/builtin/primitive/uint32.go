package primitive

import (
	"github.com/axieinfinity/steit/steit-go/pkg/path"
	"github.com/axieinfinity/steit/steit-go/pkg/reader"
)

type Uint32 uint32

func (u *Uint32) Deserialize(r reader.IReader, path *path.Path) error {
	data := reader.ReadUInt32(r)
	*u = Uint32(data)
	return nil
}
