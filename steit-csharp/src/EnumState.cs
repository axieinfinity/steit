using System;
using System.Collections.Generic;

using Steit.Reader;

namespace Steit {
    public abstract class EnumState : State {
        protected override void Replace(StateReader reader, bool shouldNotify) {
            this.ReplaceAt(reader.ReadUInt16(), StateReader.WIRE_TYPE_SIZED, reader, shouldNotify);
        }

        protected override void ReplaceAll(StateReader reader, bool shouldNotify) {
            this.Replace(reader, shouldNotify);
        }
    }
}
