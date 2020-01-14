use crate::gen::*;

pub struct CSharpGenerator {
    namespace: String,
    out_dir: String,
}

impl CSharpGenerator {
    pub fn new(namespace: impl Into<String>, out_dir: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            out_dir: out_dir.into(),
        }
    }
}

impl CSharpGenerator {
    pub fn generate_file_opening(&self, writer: &mut Writer) {
        writer
            .writeln("using System;")
            .writeln("using System.Collections.Generic;")
            .newline()
            .writeln("using Steit.Collections;")
            .writeln("using Steit.Encoding;")
            .writeln("using Steit.State;")
            .newline()
            .writeln(format!("namespace {} {{", self.namespace))
            .indent();
    }

    pub fn generate_file_closing(&self, writer: &mut Writer) {
        writer.outdent_writeln("}");
    }
}

impl Generator for CSharpGenerator {
    fn out_dir(&self) -> &str {
        &self.out_dir
    }

    fn indent_size(&self) -> usize {
        4
    }

    fn generate_struct(&self, r#struct: &Struct, is_variant: bool, writer: &mut Writer) {
        let name = r#struct.name;
        let var_name = string_utils::uncap_first_char(name);

        let fields: Vec<_> = r#struct
            .fields
            .iter()
            .map(CSharpField::with_field)
            .collect();

        if !is_variant {
            self.generate_file_opening(writer);
        }

        writer
            .writeln(format!("public sealed class {} : IState {{", name))
            .indent();

        // Declare listener lists
        for field in &fields {
            writer.writeln(format!(
                "private static IList<Listener<{0}>> {1}Listeners = new List<Listener<{0}>>();",
                field.ty, field.lower_camel_case_name,
            ));
        }

        writer
            .newline()
            .writeln("public Path Path { get; private set; }")
            .newline();

        // Declare properties
        for field in &fields {
            writer.writeln(format!(
                "public {} {} {{ get; private set; }}",
                field.ty, field.upper_camel_case_name,
            ));
        }

        writer.newline();

        if is_variant {
            writer.writeln("// This is not meant to be used directly.");
        }

        writer
            .writeln(format!("public {}(Path path = null) {{", name))
            .indent_writeln("this.Path = path != null ? path : Path.Root;");

        // Initiate nested states
        for field in &fields {
            if let FieldType::Meta(_) = field.raw.ty {
                writer.writeln(format!(
                    "this.{} = new {}(this.Path.Nested({}));",
                    field.upper_camel_case_name, field.ty, field.raw.tag,
                ));
            }
        }

        writer
            .outdent_writeln("}")
            .newline()
            .writeln(format!(
                "public delegate void Listener<T>(T newValue, T oldValue, {} container);",
                name,
            ))
            .newline();

        // Support adding listeners
        for field in &fields {
            writer.writeln(format!(
                "public static int OnUpdate{}(Listener<{}> listener) {{ return Utilities.Add({}Listeners, listener); }}",
                field.upper_camel_case_name, field.ty, field.lower_camel_case_name,
            ));
        }

        writer.newline();

        // Support removing listeners
        for field in &fields {
            writer.writeln(format!(
                "public static void Remove{}Listener(Listener<{}> listener) {{ {}Listeners.Remove(listener); }}",
                field.upper_camel_case_name, field.ty, field.lower_camel_case_name,
            ));
        }

        writer.newline();

        // Support removing listeners at specific indices
        for field in &fields {
            writer.writeln(format!(
                "public static void Remove{}ListenerAt(int index) {{ {}Listeners.RemoveAt(index); }}",
                field.upper_camel_case_name, field.lower_camel_case_name,
            ));
        }

        writer.newline();

        // Support clearing listener lists
        for field in &fields {
            writer.writeln(format!(
                "public static void Clear{}Listeners() {{ {}Listeners.Clear(); }}",
                field.upper_camel_case_name, field.lower_camel_case_name,
            ));
        }

        writer
            .newline()
            .writeln("public static void ClearAllListeners() {")
            .indent();

        // Support clearing all listener lists
        for field in &fields {
            writer.writeln(format!("{}Listeners.Clear();", field.lower_camel_case_name));
        }

        writer.outdent_writeln("}").newline();

        if is_variant {
            writer.writeln("// This is not meant to be used directly.");
        }

        writer
            .writeln(format!(
                "public static {} Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {{",
                name,
            ))
            .indent_writeln(format!("var {} = new {}(path);", var_name, name));

        if is_variant {
            writer.writeln(format!("{}.ReplaceAll(reader, shouldNotify);", var_name));
        } else {
            writer
                .newline()
                .writeln("if (!reader.Eof()) {")
                .indent_writeln(format!(
                    "{}.ReplaceAll(reader.Nested((int) reader.ReadUInt32()), shouldNotify: false);",
                    var_name,
                ))
                .outdent_writeln("}")
                .newline();
        }

        writer
            .writeln(format!("return {};", var_name))
            .outdent_writeln("}")
            .newline()
            .writeln("public Int16 WireType(UInt16 tag) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Return wire types
        for field in r#struct.fields {
            if let FieldType::Meta(_) = field.ty {
                writer.writeln(format!(
                    "case {}: return (Int16) Encoding.WireType.Sized;",
                    field.tag,
                ));
            } else {
                writer.writeln(format!(
                    "case {}: return (Int16) Encoding.WireType.Varint;",
                    field.tag,
                ));
            }
        }

        writer
            .writeln("default: return -1;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln("public IState Nested(UInt16 tag) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Return nested states
        for field in &fields {
            if let FieldType::Meta(_) = field.raw.ty {
                writer.writeln(format!(
                    "case {}: return this.{};",
                    field.raw.tag, field.upper_camel_case_name,
                ));
            }
        }

        writer
            .writeln("default: return null;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln("public bool IsAddSupported() { return false; }")
            .writeln("public bool IsRemoveSupported() { return false; }")
            .newline()
            .writeln("public void ReplayAdd(Reader reader) { throw new Exception(\"Not supported\"); }")
            .writeln("public void ReplayRemove(UInt16 tag) { throw new Exception(\"Not supported\"); }")
            .newline()
            .writeln("public void ReplaceAt(UInt16 tag, WireType wireType, Reader reader, bool shouldNotify) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Replace fields and notify listeners
        for field in &fields {
            if let FieldType::Meta(_) = field.raw.ty {
                writer.writeln(format!(
                    "case {0}: this.{1} = this.Notify({2}.Deserialize(reader, this.Path.Nested({0})), this.{1}, shouldNotify, {3}Listeners); break;",
                    field.raw.tag,
                    field.upper_camel_case_name,
                    get_type(field.raw.ty),
                    field.lower_camel_case_name,
                ));
            } else {
                writer.writeln(format!(
                    "case {0}: this.{1} = this.Notify(reader.Read{3}(), this.{1}, shouldNotify, {2}Listeners); break;",
                    field.raw.tag,
                    field.upper_camel_case_name,
                    field.lower_camel_case_name,
                    field.ty,
                ));
            }
        }

        writer
            .writeln("default: reader.SkipWireTyped(wireType); break;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln("private T Notify<T>(T newValue, T oldValue, bool shouldNotify, IList<Listener<T>> listeners) {")
            .indent_writeln("if (shouldNotify) {")
            .indent_writeln("foreach (var listener in listeners) {")
            .indent_writeln("listener(newValue, oldValue, this);")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln("return newValue;")
            .outdent_writeln("}")
            .outdent_writeln("}");

        if !is_variant {
            self.generate_file_closing(writer);
        }
    }

    fn generate_enum(&self, r#enum: &Enum, writer: &mut Writer) {
        let name = r#enum.name;
        let var_name = string_utils::uncap_first_char(name);

        let variants: Vec<_> = r#enum
            .variants
            .iter()
            .map(CSharpVariant::with_variant)
            .collect();

        let default_variant = variants
            .iter()
            .find(|variant| variant.raw.is_default())
            .expect(&format!("expect a default variant for enum {}", &name));

        self.generate_file_opening(writer);

        writer
            .writeln(format!("public sealed class {} : IEnumState {{", name))
            .indent();

        // Declare variant tag numbers
        for variant in &variants {
            writer.writeln(format!(
                "public static UInt16 {}_VARIANT = {};",
                variant.screaming_snake_case_name, variant.raw.tag,
            ));
        }

        writer
            .newline()
            .writeln("private static IList<Listener> listeners = new List<Listener>();")
            .newline()
            .writeln("public Path Path { get; private set; }")
            .newline()
            .writeln("public UInt16 Variant { get; private set; }")
            .writeln("public IState Value { get; private set; }")
            .newline();

        // Return variant values
        for variant in r#enum.variants {
            writer.writeln(format!(
                "public {0} {0}Value {{ get {{ return this.Variant == {1} ? ({0}) this.Value : null; }} }}",
                variant.ty.name, variant.tag,
            ));
        }

        writer
            .newline()
            .writeln(format!("public {}(Path path = null) {{", name))
            .indent_writeln("this.Path = path != null ? path : Path.Root;")
            .writeln(format!(
                "this.Value = new {}(this.Path.Nested({}));",
                default_variant.raw.ty.name, default_variant.raw.tag,
            ))
            .outdent_writeln("}")
            .newline()
            .writeln(format!(
                "public delegate void Listener(IState newValue, UInt16 newVariant, IState oldValue, UInt16 oldVariant, {} container);",
                name,
            ))
            .newline()
            .writeln("public static int OnUpdate(Listener listener) { return Utilities.Add(listeners, listener); }")
            .writeln("public static void RemoveListener(Listener listener) { listeners.Remove(listener); }")
            .writeln("public static void RemoveListenerAt(int index) { listeners.RemoveAt(index); }")
            .writeln("public static void ClearListeners() { listeners.Clear(); }")
            .newline()
            .writeln(format!(
                "public static {} Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {{",
                name,
            ))
            .indent_writeln(format!("var {} = new {}(path);", var_name, name))
            .writeln(format!(
                "{}.ReplaceAll(reader.Nested((int) reader.ReadUInt32()), shouldNotify);",
                var_name,
            ))
            .writeln(format!("return {};", var_name))
            .outdent_writeln("}")
            .newline()
            .writeln("public Int16 WireType(UInt16 tag) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Return wire types
        for variant in r#enum.variants {
            writer.writeln(format!(
                "case {}: return (Int16) Encoding.WireType.Sized;",
                variant.tag
            ));
        }

        writer
            .writeln("default: return -1;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln("public IState Nested(UInt16 tag) {")
            .indent_writeln("return tag == this.Variant ? this.Value : null;")
            .outdent_writeln("}")
            .newline()
            .writeln("public bool IsAddSupported() { return false; }")
            .writeln("public bool IsRemoveSupported() { return false; }")
            .newline()
            .writeln("public void ReplayAdd(Reader reader) { throw new Exception(\"Not supported\"); }")
            .writeln("public void ReplayRemove(UInt16 tag) { throw new Exception(\"Not supported\"); }")
            .newline()
            .writeln("public void ReplaceAt(UInt16 tag, WireType wireType, Reader reader, bool shouldNotify) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Replace fields and notify listeners
        for variant in r#enum.variants {
            writer.writeln(format!(
                "case {0}: this.NotifyAndUpdate({0}, {1}.Deserialize(reader, this.Path.Nested({0})), shouldNotify); break;",
                variant.tag, variant.ty.name,
            ));
        }

        writer
            .writeln("default: reader.Exhaust(); break;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln("private void NotifyAndUpdate(UInt16 newVariant, IState newValue, bool shouldNotify) {")
            .indent_writeln("if (shouldNotify) {")
            .indent_writeln("foreach (var listener in listeners) {")
            .indent_writeln("listener(newValue, newVariant, this.Value, this.Variant, this);")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln("this.Variant = newVariant;")
            .writeln("this.Value = newValue;")
            .outdent_writeln("}");

        for variant in r#enum.variants {
            writer.newline();
            self.generate_struct(variant.ty, true, writer);
        }

        writer.outdent_writeln("}");

        self.generate_file_closing(writer);
    }
}

struct CSharpVariant {
    raw: &'static Variant,
    screaming_snake_case_name: String,
}

impl CSharpVariant {
    pub fn with_variant(variant: &'static Variant) -> Self {
        Self {
            raw: variant,
            screaming_snake_case_name: string_utils::to_snake_case(variant.ty.name).to_uppercase(),
        }
    }
}

struct CSharpField {
    raw: &'static Field,
    upper_camel_case_name: String,
    lower_camel_case_name: String,
    ty: String,
}

impl CSharpField {
    pub fn with_field(field: &'static Field) -> Self {
        Self {
            raw: field,
            upper_camel_case_name: string_utils::to_camel_case(field.name, true),
            lower_camel_case_name: string_utils::to_camel_case(field.name, false),
            ty: get_type(field.ty),
        }
    }
}

fn get_type(ty: &'static FieldType) -> String {
    match ty {
        FieldType::Primitive(name) => match *name {
            "u8" => "Byte".to_owned(),
            "u16" => "UInt16".to_owned(),
            "u32" => "UInt32".to_owned(),
            "u64" => "UInt64".to_owned(),
            "i8" => "SByte".to_owned(),
            "i16" => "Int16".to_owned(),
            "i32" => "Int32".to_owned(),
            "i64" => "Int64".to_owned(),
            "bool" => "Boolean".to_owned(),
            _ => name.to_string(),
        },

        FieldType::Meta(meta) => match meta {
            Meta::Struct(Struct { name, .. }) => name.to_string(),
            Meta::Enum(Enum { name, .. }) => name.to_string(),
            Meta::List(field_type) => format!("StateList<{}>", get_type(field_type)),
            Meta::Map(field_type) => format!("StateDictionary<{}>", get_type(field_type)),
        },
    }
}
