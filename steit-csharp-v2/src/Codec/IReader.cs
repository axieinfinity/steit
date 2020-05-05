namespace Steit.Codec {
    public interface IReader {
        int Remaining();
        byte Read();
        byte[] Read(int count);
        void Skip(int count);
    }
}
