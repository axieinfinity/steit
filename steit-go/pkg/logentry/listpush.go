package logentry

type ListPush struct {
	item []byte
}

func (l *ListPush) GetItem() []byte {
	return l.item
}
