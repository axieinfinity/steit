using System.Collections.Generic;

namespace Steit {
    public static class Utilities {
        public static int Add<T>(IList<T> list, T item) {
            list.Add(item);
            return list.Count - 1;
        }
    }
}
