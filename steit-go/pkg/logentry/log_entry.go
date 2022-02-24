package logentry

import (
	"github.com/axieinfinity/steit-go/pkg/path"
	"github.com/axieinfinity/steit-go/pkg/reader"
)

type LogEntry struct {
	path            *path.Path
	tag             uint32
	listPushVariant *ListPush
	updateVariant   *Update
}

func Deserialize(r reader.IReader, path *path.Path) *LogEntry {
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
