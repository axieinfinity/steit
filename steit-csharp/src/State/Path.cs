using System;
using System.Text;

namespace Steit.State {
    public sealed class Path {
        public static Path Root = new Path(null, null);

        // public Path? Parent { get; private set; }
        public Path Parent { get; private set; }
        public UInt32? Tag { get; private set; }

        // private Path(Path? parent, UInt32? tag) {
        private Path(Path parent, UInt32? tag) {
            this.Parent = parent;
            this.Tag = tag;
        }

        public Path GetNested(UInt32 tag) {
            return new Path(this, tag);
        }

        public override string ToString() {
            var builder = new StringBuilder();
            this.ToString(builder);
            return builder.ToString();
        }

        private void ToString(StringBuilder builder) {
            if (this.Parent != null) {
                this.Parent.ToString(builder);
                builder.AppendFormat("/{0}", this.Tag);
            }
        }
    }
}
