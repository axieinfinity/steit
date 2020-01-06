use std::collections::HashMap;

use super::gen_meta::{FieldType, Meta, State};

pub fn collect_states(root: &'static Meta, states: &mut HashMap<&'static str, State>) {
    match root {
        Meta::Struct(r#struct) => {
            states.insert(r#struct.name, State::Struct(r#struct));

            for field in r#struct.fields {
                collect_states_from_field(field.ty, states);
            }
        }

        Meta::Enum(r#enum) => {
            states.insert(r#enum.name, State::Enum(r#enum.clone()));
        }

        Meta::List(field_type) => collect_states_from_field(field_type, states),
        Meta::Map(field_type) => collect_states_from_field(field_type, states),
        Meta::Rc(field_type) => collect_states_from_field(field_type, states),
    }
}

pub fn collect_states_from_field(
    field_type: &'static FieldType,
    states: &mut HashMap<&'static str, State>,
) {
    match field_type {
        FieldType::Primitive(_) => {}
        FieldType::Meta(meta) => collect_states(meta, states),
    }
}
