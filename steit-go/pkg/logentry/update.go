package logentry

type Update struct {
	value []byte
}

func (l *Update) GetValue() []byte {
	return l.value
}
