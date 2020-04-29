use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
};

use super::{
    gen_meta_v2::{EnumV2, HasMetaV2, MetaV2, StructV2},
    gen_util,
    setting::Setting,
    writer::Writer,
};

pub trait GeneratorV2 {
    const INDENT_SIZE: usize;

    type Setting;

    fn gen_struct(
        &self,
        r#struct: &StructV2,
        is_variant: bool,
        setting: &Self::Setting,
        writer: &mut Writer,
    );

    fn gen_enum(&self, r#enum: &EnumV2, setting: &Self::Setting, writer: &mut Writer);

    fn generate<T: HasMetaV2>(&self, setting: Setting<Self::Setting>) -> io::Result<()> {
        let mut all_meta = HashMap::new();

        gen_util::collect_meta_v2(T::META, &mut all_meta);

        for (name, meta) in all_meta {
            if meta.is_builtin() && setting.skip_builtins {
                continue;
            }

            let mut writer = Writer::new(Self::INDENT_SIZE);

            match meta {
                MetaV2::Struct(r#struct) => {
                    self.gen_struct(&r#struct, false, &*setting, &mut writer)
                }

                MetaV2::Enum(r#enum) => self.gen_enum(&r#enum, &*setting, &mut writer),
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
