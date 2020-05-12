use std::io;

use crate::{
    de::{Deserialize, Reader},
    log::LogEntryKind,
    meta::{HasMeta, MetaLink, NameMeta, TypeMeta},
    rt::{Runtime, SizeCache},
    ser::Serialize,
    state::State,
    wire_fmt::{HasWireType, WireType},
};

impl<T: HasWireType> HasWireType for Box<T> {
    const WIRE_TYPE: WireType = T::WIRE_TYPE;
}

impl<T: Serialize> Serialize for Box<T> {
    #[inline]
    fn compute_size(&self) -> u32 {
        self.as_ref().compute_size()
    }

    #[inline]
    fn serialize_cached(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.as_ref().serialize_cached(writer)
    }

    #[inline]
    fn size_cache(&self) -> Option<&SizeCache> {
        self.as_ref().size_cache()
    }
}

impl<T: Deserialize> Deserialize for Box<T> {
    #[inline]
    fn merge(&mut self, reader: &mut Reader<impl io::Read>) -> io::Result<()> {
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
    fn handle(
        &mut self,
        path: impl Iterator<Item = u32>,
        kind: LogEntryKind,
        reader: &mut Reader<impl io::Read>,
    ) -> io::Result<()> {
        self.as_mut().handle(path, kind, reader)
    }
}

impl<T: HasMeta> HasMeta for Box<T> {
    const NAME: &'static NameMeta = &NameMeta {
        rust: "Box",
        csharp: Some("Box"),
    };

    const TYPE: &'static TypeMeta = T::TYPE;

    const LINK: &'static MetaLink = &MetaLink {
        r#type: Self::TYPE,
        msg: None,
        links: || &[T::LINK],
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        ser::Serialize,
        test_case,
        test_util::{assert_merge, assert_serialize, assert_serialize_nested, assert_size, Foo},
    };

    #[test]
    fn cached_size() {
        let value = Some(Foo::new(-1, 0));
        assert_eq!(value.as_ref().unwrap().cached_size(), 0);
        assert_eq!(value.cache_size(), 3);
        assert_eq!(value.unwrap().cached_size(), 2);
    }

    test_case!(size_01: assert_size; Box::new(0) => 1);
    test_case!(size_02: assert_size; Box::new(1) => 1);
    test_case!(size_03: assert_size; Box::new(1337) => 2);

    test_case!(serialize_01: assert_serialize; None::<u8> => &[]);
    test_case!(serialize_02: assert_serialize; Some(0) => &[0]);
    test_case!(serialize_03: assert_serialize; Some(1337) => &[242, 20]);
    test_case!(serialize_04: assert_serialize; Some(Foo::empty()) => &[0]);
    test_case!(serialize_05: assert_serialize; Some(Foo::new(-1, -2)) => &[4, 0, 1, 8, 3]);

    test_case!(serialize_nested_01: assert_serialize_nested; None::<u8>, None => &[0]);
    test_case!(serialize_nested_02: assert_serialize_nested; Some(1), None => &[1, 2]);
    test_case!(serialize_nested_03: assert_serialize_nested; None::<u8>, Some(10) => &[]);
    test_case!(serialize_nested_04: assert_serialize_nested; Some(1), Some(10) => &[82, 1, 2]);
    test_case!(serialize_nested_05: assert_serialize_nested; Some(Foo::empty()), Some(10) => &[82, 1, 0]);
    test_case!(serialize_nested_06: assert_serialize_nested; Some(Foo::new(-1, -2)), Some(10) => &[82, 5, 4, 0, 1, 8, 3]);

    test_case!(merge_01: assert_merge; Some(1), &[] => Some(1));
    test_case!(merge_02: assert_merge; None, &[0] => Some(0));
    test_case!(merge_03: assert_merge; Some(0), &[242, 20] => Some(1337));
    test_case!(merge_04: assert_merge; Some(Foo::new(-1, -2)), &[2, 8, 4] => Some(Foo::new(-1, 2)));
    test_case!(merge_05: assert_merge; None, &[2, 8, 4] => Some(Foo::new(0, 2)));
}
