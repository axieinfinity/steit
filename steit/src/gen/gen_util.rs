use std::collections::{HashMap, HashSet};

use crate::meta::{HasMeta, MessageMeta, MetaLink, TypeMeta};

pub fn collect_meta<T: HasMeta>(
    get_name: fn(&'static MessageMeta) -> String,
) -> HashMap<String, &'static MessageMeta> {
    let mut visited_types = HashSet::new();
    let mut collected_msgs = HashMap::new();
    visit_link(T::LINK, get_name, &mut visited_types, &mut collected_msgs);
    collected_msgs
}

fn visit_link(
    entry: &'static MetaLink,
    get_name: fn(&'static MessageMeta) -> String,
    visited_types: &mut HashSet<&'static TypeMeta>,
    collected_msgs: &mut HashMap<String, &'static MessageMeta>,
) {
    if let Some(msg) = &entry.msg {
        collected_msgs.entry(get_name(msg)).or_insert(msg);
    }

    if !visited_types.contains(entry.r#type) {
        visited_types.insert(entry.r#type);

        for &link in (entry.links)() {
            visit_link(link, get_name, visited_types, collected_msgs);
        }
    }
}
