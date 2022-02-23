package codec

type Path struct {
	path *Path
}

func (p *Path) GetNested(tag uint32) *Path {
	return nil
}
