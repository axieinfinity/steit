package logentry

import (
	"github.com/axieinfinity/steit-go/pkg/codec"
	"github.com/axieinfinity/steit-go/pkg/reader"
)

type LogEntry struct {
	path            *codec.Path
	tag             uint32
	listPushVariant *ListPush
	updateVariant   *Update
}

func Deserialize(r reader.IReader, path *codec.Path) *LogEntry {
	logEntry := &LogEntry{path: path}
	return logEntry
}

func (e *LogEntry) GetTag() uint32 {
	return e.tag
}

func (e *LogEntry) GetListPushVariant() *ListPush {
	return e.listPushVariant
}

func (e *LogEntry) GetUpdateVariant() *Update {
	return e.updateVariant
}
