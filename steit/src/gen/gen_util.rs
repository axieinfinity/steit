use std::collections::HashMap;

use crate::meta::{MessageMeta, TypeMeta};

use super::gen_meta::{FieldType, Meta};

pub fn collect_meta(root: &'static Meta, all_meta: &mut HashMap<&'static str, Meta>) {
    match *root {
        Meta::Struct(r#struct) => {
            all_meta.insert(r#struct.name, Meta::Struct(r#struct));

            for field in r#struct.fields {
                collect_meta_from_field(field.ty, all_meta);
            }
        }

        Meta::Enum(r#enum) => {
            all_meta.insert(r#enum.name, Meta::Enum(r#enum));

            for variant in r#enum.variants {
                for field in variant.ty.fields {
                    collect_meta_from_field(field.ty, all_meta);
                }
            }
        }
    }
}

fn collect_meta_from_field(
    field_type: &'static FieldType,
    all_meta: &mut HashMap<&'static str, Meta>,
) {
    match *field_type {
        FieldType::Primitive(_) => (),
        FieldType::Meta(meta) => collect_meta(meta, all_meta),
        FieldType::MetaRef(_) => (),
        FieldType::Bytes => (),
        FieldType::List(field_type) | FieldType::Map(field_type) | FieldType::Vec(field_type) => {
            collect_meta_from_field(field_type, all_meta)
        }
    }
}

pub fn collect_meta_v2(
    root: &'static MessageMeta,
    all_meta: &mut HashMap<&'static str, MessageMeta>,
) {
    match *root {
        MessageMeta::Struct(r#struct) => {
            all_meta.insert(r#struct.name, MessageMeta::Struct(r#struct));

            for field in r#struct.fields {
                collect_meta_from_field_v2(field.ty, all_meta);
            }
        }

        MessageMeta::Enum(r#enum) => {
            all_meta.insert(r#enum.name, MessageMeta::Enum(r#enum));

            for variant in r#enum.variants {
                for field in variant.ty.fields {
                    collect_meta_from_field_v2(field.ty, all_meta);
                }
            }
        }
    }
}

fn collect_meta_from_field_v2(
    field_type: &'static TypeMeta,
    all_meta: &mut HashMap<&'static str, MessageMeta>,
) {
    match *field_type {
        TypeMeta::Primitive(_) => (),
        TypeMeta::Message(meta) => collect_meta_v2(meta, all_meta),
        TypeMeta::MessageRef(_) => (),
        TypeMeta::Vec(field_type) | TypeMeta::List(field_type) | TypeMeta::Map(field_type) => {
            collect_meta_from_field_v2(field_type, all_meta)
        }
    }
}
