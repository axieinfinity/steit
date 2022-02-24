package state

import (
	"github.com/axieinfinity/steit-go/pkg/path"
	"github.com/axieinfinity/steit-go/pkg/reader"
)

func Deserialize(r reader.IReader, path *path.Path) IState {
	return nil
}

func DeserializeNested(r reader.IReader, path *path.Path, tag uint32) interface{} {
	return nil
}
