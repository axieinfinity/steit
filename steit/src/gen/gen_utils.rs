use std::collections::HashMap;

use super::gen_meta::{FieldType, Meta};

pub fn collect_meta(root: &'static Meta, meta_list: &mut HashMap<&'static str, Meta>) {
    match root {
        Meta::Struct(r#struct) => {
            meta_list.insert(r#struct.name, Meta::Struct(r#struct));

            for field in r#struct.fields {
                collect_meta_from_field(field.ty, meta_list);
            }
        }

        Meta::Enum(r#enum) => {
            meta_list.insert(r#enum.name, Meta::Enum(r#enum.clone()));

            for variant in r#enum.variants {
                for field in variant.ty.fields {
                    collect_meta_from_field(field.ty, meta_list);
                }
            }
        }
    }
}

fn collect_meta_from_field(
    field_type: &'static FieldType,
    meta_list: &mut HashMap<&'static str, Meta>,
) {
    match field_type {
        FieldType::Primitive(_) => {}
        FieldType::Meta(meta) => collect_meta(meta, meta_list),
        FieldType::MetaRef(_) => {}
        FieldType::List(field_type) | FieldType::Map(field_type) | FieldType::Vec(field_type) => {
            collect_meta_from_field(field_type, meta_list)
        }
    }
}
