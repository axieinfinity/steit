use std::io;

use crate::{
    gen::{FieldType, IsFieldType},
    wire_type::WireType,
    Eof, Merge, ReplayKind, Runtime, Serialize, State,
};

impl<T: WireType> WireType for Box<T> {
    const WIRE_TYPE: u8 = T::WIRE_TYPE;
}

impl<T: Serialize> Serialize for Box<T> {
    #[inline]
    fn compute_size(&self) -> u32 {
        self.as_ref().compute_size()
    }

    #[inline]
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.as_ref().serialize_with_cached_size(writer)
    }

    #[inline]
    fn cached_size(&self) -> u32 {
        self.as_ref().cached_size()
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.as_ref().serialize(writer)
    }
}

impl<T: Merge> Merge for Box<T> {
    #[inline]
    fn merge(&mut self, reader: &mut Eof<impl io::Read>) -> io::Result<()> {
        self.as_mut().merge(reader)
    }
}

impl<T: State> State for Box<T> {
    #[inline]
    fn with_runtime(runtime: Runtime) -> Self {
        Self::new(T::with_runtime(runtime))
    }

    #[inline]
    fn runtime(&self) -> &Runtime {
        self.as_ref().runtime()
    }

    #[inline]
    fn set_runtime(&mut self, runtime: Runtime) {
        self.as_mut().set_runtime(runtime)
    }

    #[inline]
    fn handle<'a>(
        &mut self,
        path: &mut impl Iterator<Item = &'a u16>,
        kind: ReplayKind,
        reader: &mut Eof<impl io::Read>,
    ) -> io::Result<()> {
        self.as_mut().handle(path, kind, reader)
    }
}

impl<T: IsFieldType> IsFieldType for Box<T> {
    const FIELD_TYPE: &'static FieldType = T::FIELD_TYPE_REF;
}

#[cfg(test)]
mod tests {
    use crate::{
        test_case,
        test_util::{assert_merge, assert_serialize, assert_serialize_nested, assert_size, Foo},
        Serialize,
    };

    #[test]
    fn size_cache() {
        let value = Some(Foo::with(-1, 0));
        assert_eq!(value.as_ref().unwrap().cached_size(), 0);
        assert_eq!(value.cached_size(), 3);
        assert_eq!(value.unwrap().cached_size(), 2);
    }

    test_case!(size_01: assert_size; Box::new(0) => 1);
    test_case!(size_02: assert_size; Box::new(1) => 1);
    test_case!(size_03: assert_size; Box::new(1337) => 2);

    test_case!(serialize_01: assert_serialize; None::<u8> => &[]);
    test_case!(serialize_02: assert_serialize; Some(0) => &[0]);
    test_case!(serialize_03: assert_serialize; Some(1337) => &[242, 20]);
    test_case!(serialize_04: assert_serialize; Some(Foo::new()) => &[0]);
    test_case!(serialize_05: assert_serialize; Some(Foo::with(-1, -2)) => &[4, 0, 1, 8, 3]);

    test_case!(serialize_nested_01: assert_serialize_nested; None::<u8>, None => &[0]);
    test_case!(serialize_nested_02: assert_serialize_nested; Some(1), None => &[1, 2]);
    test_case!(serialize_nested_03: assert_serialize_nested; None::<u8>, Some(10) => &[]);
    test_case!(serialize_nested_04: assert_serialize_nested; Some(1), Some(10) => &[82, 1, 2]);
    test_case!(serialize_nested_05: assert_serialize_nested; Some(Foo::new()), Some(10) => &[82, 1, 0]);
    test_case!(serialize_nested_06: assert_serialize_nested; Some(Foo::with(-1, -2)), Some(10) => &[82, 5, 4, 0, 1, 8, 3]);

    test_case!(merge_01: assert_merge; Some(1), &[] => Some(1));
    test_case!(merge_02: assert_merge; None, &[0] => Some(0));
    test_case!(merge_03: assert_merge; Some(0), &[242, 20] => Some(1337));
    test_case!(merge_04: assert_merge; Some(Foo::with(-1, -2)), &[2, 8, 4] => Some(Foo::with(-1, 2)));
    test_case!(merge_05: assert_merge; None, &[2, 8, 4] => Some(Foo::with(0, 2)));
}
