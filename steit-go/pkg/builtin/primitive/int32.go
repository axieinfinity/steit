package primitive

import (
	"github.com/axieinfinity/steit-go/pkg/path"
	"github.com/axieinfinity/steit-go/pkg/reader"
)

type Int32 int32

func (u *Int32) Deserialize(r reader.IReader, path *path.Path) error {
	data := reader.ReadInt32(r)
	*u = Int32(data)
	return nil
}
