use std::{collections::HashMap, fs::File, io, io::Write, path::Path};

use super::{
    gen_meta::{Field, HasMeta},
    gen_utils,
};

pub trait Generator {
    fn out_dir(&self) -> &str;
    fn generate_state(&self, name: &'static str, fields: &'static [Field]) -> String;

    fn generate<T: HasMeta>(&self) -> io::Result<()> {
        let mut states = HashMap::new();

        gen_utils::collect_states(T::META, &mut states);

        for (name, state) in states {
            let source = self.generate_state(name, state);

            let path = Path::new(self.out_dir()).join(format!("{}.cs", name));
            let file = File::create(path)?;
            let mut writer = io::BufWriter::new(file);

            writer.write_all(source.as_bytes())?;
        }

        Ok(())
    }
}
