use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
};

use crate::meta::{EnumMeta, HasMessageMeta, MessageMeta, StructMeta};

use super::{gen_util, setting::Setting, writer::Writer};

pub trait GeneratorV2 {
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

    fn generate<T: HasMessageMeta>(&self, setting: Setting<Self::Setting>) -> io::Result<()> {
        let mut meta_map = HashMap::new();

        gen_util::collect_meta_v2(T::MESSAGE_META, &mut meta_map);

        for (name, meta) in meta_map {
            if meta.is_builtin() && setting.skip_builtins {
                continue;
            }

            let mut writer = Writer::new(Self::INDENT_SIZE);

            match meta {
                MessageMeta::Struct(r#struct) => {
                    self.gen_struct(&r#struct, false, &*setting, &mut writer)
                }

                MessageMeta::Enum(r#enum) => self.gen_enum(&r#enum, &*setting, &mut writer),
            };

            let source = writer.end();

            let path = setting.out_dir.join(format!("{}.cs", name));
            let file = File::create(path)?;

            let mut writer = io::BufWriter::new(file);

            writer.write_all(source.as_bytes())?;
        }

        Ok(())
    }
}
