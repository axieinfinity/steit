using System;
using System.Collections.Generic;

namespace Steit {
    public static class Utils {
        public static int Add<T>(IList<T> list, T item) {
            list.Add(item);
            return list.Count - 1;
        }
    }
}
