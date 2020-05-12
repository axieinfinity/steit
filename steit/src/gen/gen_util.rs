use std::collections::{HashMap, HashSet};

use crate::meta::{HasMeta, MessageMeta, MetaLink, TypeMeta};

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
