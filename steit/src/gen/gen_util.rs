use std::collections::HashMap;

use super::gen_meta::{FieldType, Meta};
use crate::gen::{FieldTypeV2, MetaV2};

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

pub fn collect_meta_v2(root: &'static MetaV2, all_meta: &mut HashMap<&'static str, MetaV2>) {
    match *root {
        MetaV2::Struct(r#struct) => {
            all_meta.insert(r#struct.name, MetaV2::Struct(r#struct));

            for field in r#struct.fields {
                collect_meta_from_field_v2(field.ty, all_meta);
            }
        }

        MetaV2::Enum(r#enum) => {
            all_meta.insert(r#enum.name, MetaV2::Enum(r#enum));

            for variant in r#enum.variants {
                for field in variant.ty.fields {
                    collect_meta_from_field_v2(field.ty, all_meta);
                }
            }
        }
    }
}

fn collect_meta_from_field_v2(
    field_type: &'static FieldTypeV2,
    all_meta: &mut HashMap<&'static str, MetaV2>,
) {
    match *field_type {
        FieldTypeV2::Primitive(_) => (),
        FieldTypeV2::Meta(meta) => collect_meta_v2(meta, all_meta),
        FieldTypeV2::MetaRef(_) => (),
        FieldTypeV2::Vec(field_type)
        | FieldTypeV2::List(field_type)
        | FieldTypeV2::Map(field_type) => collect_meta_from_field_v2(field_type, all_meta),
    }
}
