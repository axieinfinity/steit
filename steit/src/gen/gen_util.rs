use std::collections::{HashMap, HashSet};

use crate::meta::{HasMeta, MessageMeta, MetaLink, TypeMeta};

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

pub fn collect_meta_v2<T: HasMeta>() -> HashMap<&'static str, &'static MessageMeta> {
    let mut visited_types = HashSet::new();
    let mut collected_msgs = HashMap::new();
    visit_link(T::LINK, &mut visited_types, &mut collected_msgs);
    collected_msgs
}

fn visit_link(
    entry: &'static MetaLink,
    visited_types: &mut HashSet<&'static TypeMeta>,
    collected_msgs: &mut HashMap<&'static str, &'static MessageMeta>,
) {
    if let Some(msg) = &entry.msg {
        let rust_name = msg.rust_name();

        if !collected_msgs.contains_key(rust_name) {
            collected_msgs.insert(rust_name, msg);
        }
    }

    if !visited_types.contains(entry.r#type) {
        visited_types.insert(entry.r#type);

        for &link in (entry.links)() {
            visit_link(link, visited_types, collected_msgs);
        }
    }
}
