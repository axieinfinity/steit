use std::collections::HashMap;

use super::gen_meta::{Field, FieldType, Meta};

pub fn collect_states(root: &'static Meta, states: &mut HashMap<&'static str, &'static [Field]>) {
    match root {
        Meta::State(name, fields) => {
            states.insert(name, fields);

            for field in *fields {
                collect_states_from_field(field, states);
            }
        }
        Meta::List(field) => collect_states_from_field(field, states),
        Meta::Map(field) => collect_states_from_field(field, states),
    }
}

pub fn collect_states_from_field(
    field: &'static Field,
    states: &mut HashMap<&'static str, &'static [Field]>,
) {
    match field.ty {
        FieldType::Primitive(_) => {}
        FieldType::Meta(meta) => collect_states(meta, states),
    }
}
