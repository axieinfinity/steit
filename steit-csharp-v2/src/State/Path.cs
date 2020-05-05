using System;
using System.Text;

namespace Steit.State {
    public sealed class Path {
        public static Path Root = new Path(null, null);

        public Path? Parent { get; private set; }
        public UInt32? Tag { get; private set; }

        private Path(Path? parent, UInt32? tag) {
            this.Parent = parent;
            this.Tag = tag;
        }

        public Path GetNested(UInt32 tag) {
            return new Path(this, tag);
        }

        private void BuildString(StringBuilder builder) {
            if (this.Parent != null) {
                this.Parent.BuildString(builder);
                builder.AppendFormat("/{0}", this.Tag);
            }
        }

        public override string ToString() {
            var builder = new StringBuilder();
            this.BuildString(builder);
            return builder.ToString();
        }
    }
}
