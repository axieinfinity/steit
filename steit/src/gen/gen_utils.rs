use std::collections::HashMap;

use super::gen_meta::{Field, FieldType, Meta, State};

pub fn collect_states(root: &'static Meta, states: &mut HashMap<&'static str, State>) {
    match root {
        Meta::Struct(r#struct) => {
            states.insert(r#struct.name, State::Struct(r#struct.clone()));

            for field in r#struct.fields {
                collect_states_from_field(field, states);
            }
        }

        Meta::Enum(r#enum) => {
            states.insert(r#enum.name, State::Enum(r#enum.clone()));
        }

        Meta::List(field) => collect_states_from_field(field, states),
        Meta::Map(field) => collect_states_from_field(field, states),
    }
}

pub fn collect_states_from_field(field: &'static Field, states: &mut HashMap<&'static str, State>) {
    match field.ty {
        FieldType::Primitive(_) => {}
        FieldType::Meta(meta) => collect_states(meta, states),
    }
}
