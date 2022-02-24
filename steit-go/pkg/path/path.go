package path

import (
	"fmt"
	"strings"
)

type Path struct {
	parent *Path
	tag    *uint32
}

var (
	Root = &Path{nil, nil}
)

func (p *Path) GetParent() *Path {
	return p.parent
}

func (p *Path) GetTag() *uint32 {
	return p.tag
}

func (p *Path) GetNested(tag uint32) *Path {
	return &Path{parent: p, tag: &tag}
}

func (p *Path) String() string {
	var builder strings.Builder
	p.buildPath(&builder)
	return builder.String()
}

func (p *Path) buildPath(builder *strings.Builder) {
	if p.parent != nil {
		p.parent.buildPath(builder)
		builder.WriteString(fmt.Sprintf("/%d", *p.tag))
	}
}
