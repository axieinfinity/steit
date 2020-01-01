namespace Steit.Encoding {
    public interface IReader {
        bool Eof();
        byte Read();
        void Skip(int length);
    }
}
