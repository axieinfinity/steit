use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
    path::Path,
};

use super::{
    gen_meta::{Enum, HasMeta, Meta, Struct},
    gen_util,
    writer::Writer,
};

pub trait Generator {
    fn out_dir(&self) -> &str;

    fn indent_size(&self) -> usize;

    fn generate_struct(&self, r#struct: &Struct, is_variant: bool, writer: &mut Writer);
    fn generate_enum(&self, r#enum: &Enum, writer: &mut Writer);

    fn generate<T: HasMeta>(&self) -> io::Result<()> {
        let mut states = HashMap::new();

        gen_util::collect_meta(T::META, &mut states);

        for (name, state) in states {
            let mut writer = Writer::new(self.indent_size());

            match state {
                Meta::Struct(r#struct) => self.generate_struct(&r#struct, false, &mut writer),
                Meta::Enum(r#enum) => self.generate_enum(&r#enum, &mut writer),
            };

            let source = writer.end();

            let path = Path::new(self.out_dir()).join(format!("{}.cs", name));
            let file = File::create(path)?;

            let mut writer = io::BufWriter::new(file);

            writer.write_all(source.as_bytes())?;
        }

        Ok(())
    }
}
