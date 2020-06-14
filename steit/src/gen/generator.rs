use std::{
    fs::File,
    io::{self, Write},
};

use crate::meta::{EnumMeta, HasMeta, MessageMeta, StructMeta};

use super::{gen_util, setting::Setting, writer::Writer};

pub trait Generator {
    const INDENT_SIZE: usize;

    type Setting;

    fn gen_struct(
        &self,
        r#struct: &StructMeta,
        is_variant: bool,
        setting: &Self::Setting,
        writer: &mut Writer,
    );

    fn gen_enum(&self, r#enum: &EnumMeta, setting: &Self::Setting, writer: &mut Writer);

    fn generate<T: HasMeta>(&self, setting: &Setting<Self::Setting>) -> io::Result<Vec<String>> {
        let mut generated_names = Vec::new();

        for (name, meta) in gen_util::collect_meta::<T>(setting.get_name) {
            if meta.is_builtin() && setting.skip_builtins {
                continue;
            }

            if setting.skip_names.contains(&name) {
                continue;
            }

            let mut writer = Writer::new(Self::INDENT_SIZE);

            match meta {
                MessageMeta::Struct(r#struct) => {
                    self.gen_struct(&r#struct, false, &*setting, &mut writer);
                }

                MessageMeta::Enum(r#enum) => {
                    self.gen_enum(&r#enum, &*setting, &mut writer);
                }
            };

            let source = writer.end();

            let path = setting.out_dir.join(format!("{}.cs", name));
            let file = File::create(path)?;

            let mut writer = io::BufWriter::new(file);

            writer.write_all(source.as_bytes())?;
            generated_names.push(name);
        }

        Ok(generated_names)
    }
}
