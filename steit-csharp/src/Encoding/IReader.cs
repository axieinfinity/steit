namespace Steit.Encoding {
    public abstract class IReader {
        public abstract bool Eof();
        public abstract int Remaining();
        public abstract byte Read();
        public abstract void Skip(int length);
        public void Exhaust() { this.Skip(this.Remaining()); }
    }
}
