use super::name::NameMeta;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum FieldTypeMeta {
    Type(&'static TypeMeta),
    TypeParam(&'static str),
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum TypeMeta {
    Primitive(&'static NameMeta),
    // In our scope, a type argument holds the same content as a field type.
    Ref(&'static NameMeta, &'static [FieldTypeMeta]),
}
