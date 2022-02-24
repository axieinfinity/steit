package path

import "testing"

func TestPath_String(t *testing.T) {
	type fields struct {
		parent *Path
		tag    *uint32
	}

	parentTag := uint32(1)
	tag := uint32(2)
	tests := []struct {
		name   string
		fields fields
		want   string
	}{
		{
			name: "Happy case",
			fields: fields{
				parent: &Path{parent: Root, tag: &parentTag},
				tag:    &tag,
			},
			want: "/1/2",
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			p := &Path{
				parent: tt.fields.parent,
				tag:    tt.fields.tag,
			}
			if got := p.String(); got != tt.want {
				t.Errorf("Path.String() = %v, want %v", got, tt.want)
			}
		})
	}
}
