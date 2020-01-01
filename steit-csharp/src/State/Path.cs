using System;

namespace Steit.State {
    public sealed class Path {
        public static Path Root = new Path(null, 0);

        public Path Parent { get; private set; }
        public UInt16 Tag { get; private set; }

        private Path(Path parent, UInt16 tag) {
            this.Parent = parent;
            this.Tag = tag;
        }

        public Path Nested(UInt16 tag) {
            return new Path(this, tag);
        }
    }
}
