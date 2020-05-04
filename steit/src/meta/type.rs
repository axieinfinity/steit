use super::name::NameMeta;

#[derive(PartialEq, Debug)]
pub enum FieldTypeMeta {
    Type(&'static TypeMeta),
    TypeParam(&'static str),
}

#[derive(PartialEq, Debug)]
pub enum TypeMeta {
    Primitive(&'static NameMeta),
    // In our scope, a type argument holds the same content as a field type.
    Ref(&'static NameMeta, &'static [FieldTypeMeta]),
}
