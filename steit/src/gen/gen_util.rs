use std::collections::HashMap;

use crate::meta::{MessageMeta, TypeMeta};

use super::gen_meta::{FieldType, Meta};

pub fn collect_meta(root: &'static Meta, meta_map: &mut HashMap<&'static str, Meta>) {
    match *root {
        Meta::Struct(r#struct) => {
            meta_map.insert(r#struct.name, Meta::Struct(r#struct));

            for field in r#struct.fields {
                collect_meta_from_field(field.ty, meta_map);
            }
        }

        Meta::Enum(r#enum) => {
            meta_map.insert(r#enum.name, Meta::Enum(r#enum));

            for variant in r#enum.variants {
                for field in variant.ty.fields {
                    collect_meta_from_field(field.ty, meta_map);
                }
            }
        }
    }
}

fn collect_meta_from_field(
    field_type: &'static FieldType,
    meta_map: &mut HashMap<&'static str, Meta>,
) {
    match *field_type {
        FieldType::Primitive(_) => (),
        FieldType::Meta(meta) => collect_meta(meta, meta_map),
        FieldType::MetaRef(_) => (),
        FieldType::Bytes => (),
        FieldType::List(field_type) | FieldType::Map(field_type) | FieldType::Vec(field_type) => {
            collect_meta_from_field(field_type, meta_map)
        }
    }
}

/*
pub fn collect_meta_v2(
    root: &'static MessageMeta,
    meta_map: &mut HashMap<&'static str, MessageMeta>,
) {
    match *root {
        MessageMeta::Struct(r#struct) => {
            meta_map.insert(r#struct.name.rust, MessageMeta::Struct(r#struct));

            for field in r#struct.fields {
                collect_meta_from_field_v2(field.ty, meta_map);
            }
        }

        MessageMeta::Enum(r#enum) => {
            meta_map.insert(r#enum.name.rust, MessageMeta::Enum(r#enum));

            for variant in r#enum.variants {
                for field in variant.ty.fields {
                    collect_meta_from_field_v2(field.ty, meta_map);
                }
            }
        }
    }
}

fn collect_meta_from_field_v2(
    field_type: &'static TypeMetaV2,
    meta_map: &mut HashMap<&'static str, MessageMeta>,
) {
    match *field_type {
        TypeMeta::Primitive(_) => (),
        TypeMeta::Message(meta) => collect_meta_v2(meta, meta_map),
        TypeMeta::MessageRef(_) => (),
        TypeMeta::Vec(field_type) | TypeMeta::List(field_type) | TypeMeta::Map(field_type) => {
            collect_meta_from_field_v2(field_type, meta_map)
        }
    }
}
*/
