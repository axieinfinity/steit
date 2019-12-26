using System;

namespace Steit.Reader {
    public interface IReader {
        bool Eof();
        byte Read();
        void Skip(int length);
    }
}
