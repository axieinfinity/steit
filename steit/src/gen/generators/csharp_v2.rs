use crate::gen::*;

pub struct CSharpSetting {
    namespace: String,
}

impl CSharpSetting {
    #[inline]
    pub fn new(namespace: String) -> Self {
        Self { namespace }
    }
}

pub struct CSharpGeneratorV2;

impl CSharpGeneratorV2 {
    pub fn gen_file_opening(&self, setting: &<Self as GeneratorV2>::Setting, writer: &mut Writer) {
        writer
            .writeln("using System;")
            .writeln("using System.Collections.Generic;")
            .newline()
            .writeln("using Steit;")
            .writeln("using Steit.Builtins;")
            .writeln("using Steit.Collections;")
            .writeln("using Steit.Encoding;")
            .writeln("using Steit.State;")
            .newline()
            .writeln(format!("namespace {} {{", &setting.namespace))
            .indent();
    }

    pub fn gen_file_closing(&self, writer: &mut Writer) {
        writer.outdent_writeln("}");
    }
}

impl GeneratorV2 for CSharpGeneratorV2 {
    const INDENT_SIZE: usize = 4;

    type Setting = CSharpSetting;

    fn gen_struct(
        &self,
        r#struct: &StructV2,
        is_variant: bool,
        setting: &Self::Setting,
        writer: &mut Writer,
    ) {
        let name = r#struct.name;
        let var_name = str_util::uncap_first_char(name);

        let fields: Vec<_> = r#struct.fields.iter().map(CSharpField::from_meta).collect();

        let variant_accessibility = if is_variant { "internal" } else { "public" };

        if !is_variant {
            self.gen_file_opening(setting, writer);
        }

        writer
            .writeln(format!("public sealed class {} : IState {{", name))
            .indent();

        // Declare listener lists
        for field in &fields {
            writer.writeln(format!(
                "private static IList<Listener<{0}>> {1}Listeners = new List<Listener<{0}>>();",
                field.type_name, field.lower_camel_case_name,
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
                field.type_name, field.upper_camel_case_name,
            ));
        }

        writer
            .newline()
            .writeln(format!(
                "{} {}(Path path = null) {{",
                variant_accessibility, name,
            ))
            .indent_writeln("this.Path = path != null ? path : Path.Root;");

        // Initiate nested states
        for field in &fields {
            if let FieldTypeV2::Primitive(_) = field.meta.ty {
            } else {
                writer.writeln(format!(
                    "this.{} = new {}(this.Path.Nested({}));",
                    field.upper_camel_case_name, field.type_name, field.meta.tag,
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
                field.upper_camel_case_name, field.type_name, field.lower_camel_case_name,
            ));
        }

        if !fields.is_empty() {
            writer.newline();
        }

        // Support removing listeners
        for field in &fields {
            writer.writeln(format!(
                "public static void Remove{}Listener(Listener<{}> listener) {{ {}Listeners.Remove(listener); }}",
                field.upper_camel_case_name, field.type_name, field.lower_camel_case_name,
            ));
        }

        if !fields.is_empty() {
            writer.newline();
        }

        // Support removing listeners at specific indices
        for field in &fields {
            writer.writeln(format!(
                "public static void Remove{}ListenerAt(int index) {{ {}Listeners.RemoveAt(index); }}",
                field.upper_camel_case_name, field.lower_camel_case_name,
            ));
        }

        if !fields.is_empty() {
            writer.newline();
        }

        // Support clearing listener lists
        for field in &fields {
            writer.writeln(format!(
                "public static void Clear{}Listeners() {{ {}Listeners.Clear(); }}",
                field.upper_camel_case_name, field.lower_camel_case_name,
            ));
        }

        if !fields.is_empty() {
            writer.newline();
        }

        writer
            .writeln("public static void ClearAllListeners() {")
            .indent();

        // Support clearing all listener lists
        for field in &fields {
            writer.writeln(format!("{}Listeners.Clear();", field.lower_camel_case_name));
        }

        writer
            .outdent_writeln("}")
            .newline()
            .writeln(format!(
                "{} static {} Deserialize(Reader reader, Path path = null, bool shouldNotify = false) {{",
                variant_accessibility,
                name,
            ))
            .indent_writeln(format!("var {} = new {}(path);", var_name, name))
            .writeln(format!("{}.ReplaceAll(reader, shouldNotify);", var_name))
            .writeln(format!("return {};", var_name))
            .outdent_writeln("}")
            .newline()
            .writeln("public Int16 WireType(UInt16 tag) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Return wire types
        for field in r#struct.fields {
            if let FieldTypeV2::Primitive(_) = field.ty {
                writer.writeln(format!(
                    "case {}: return (Int16) Steit.Encoding.WireType.Varint;",
                    field.tag,
                ));
            } else {
                writer.writeln(format!(
                    "case {}: return (Int16) Steit.Encoding.WireType.Sized;",
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
            if let FieldTypeV2::Primitive(_) = field.meta.ty {
            } else {
                writer.writeln(format!(
                    "case {}: return this.{};",
                    field.meta.tag, field.upper_camel_case_name,
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
            if let FieldTypeV2::Primitive(_) = field.meta.ty {
                writer.writeln(format!(
                    "case {0}: this.{1} = this.Notify(reader.Read{3}(), this.{1}, shouldNotify, {2}Listeners); break;",
                    field.meta.tag,
                    field.upper_camel_case_name,
                    field.lower_camel_case_name,
                    field.type_name,
                ));
            } else {
                writer.writeln(format!(
                    "case {0}: this.{1} = this.Notify({2}.Deserialize(reader.Nested(), this.Path.Nested({0})), this.{1}, shouldNotify, {3}Listeners); break;",
                    field.meta.tag,
                    field.upper_camel_case_name,
                    get_type(field.meta.ty),
                    field.lower_camel_case_name,
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
            self.gen_file_closing(writer);
        }
    }

    fn gen_enum(&self, r#enum: &EnumV2, setting: &Self::Setting, writer: &mut Writer) {
        let name = r#enum.name;
        let var_name = str_util::uncap_first_char(name);

        let variants: Vec<_> = r#enum
            .variants
            .iter()
            .map(CSharpVariant::from_meta)
            .collect();

        let default_variant = variants
            .iter()
            .find(|variant| variant.meta.default)
            .expect(&format!("expect a default variant for enum {}", name));

        self.gen_file_opening(setting, writer);

        writer
            .writeln(format!("public sealed class {} : IEnumState {{", name))
            .indent();

        // Declare variant tag numbers
        for variant in &variants {
            writer.writeln(format!(
                "public static UInt16 {}_VARIANT = {};",
                variant.screaming_snake_case_name, variant.meta.tag,
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
                default_variant.meta.ty.name, default_variant.meta.tag,
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
            .writeln(format!("{}.ReplaceAll(reader, shouldNotify);", var_name))
            .writeln(format!("return {};", var_name))
            .outdent_writeln("}")
            .newline()
            .writeln("public Int16 WireType(UInt16 tag) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Return wire types
        for variant in r#enum.variants {
            writer.writeln(format!(
                "case {}: return (Int16) Steit.Encoding.WireType.Sized;",
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
            .indent_writeln("reader = !reader.Eof() ? reader : new Reader(new byte[0]);")
            .newline()
            .writeln("switch (tag) {")
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
            self.gen_struct(variant.ty, true, setting, writer);
        }

        writer.outdent_writeln("}");

        self.gen_file_closing(writer);
    }
}

struct CSharpVariant {
    meta: &'static VariantV2,
    // SCREAMING_SNAKE_CASE
    screaming_snake_case_name: String,
}

impl CSharpVariant {
    #[inline]
    pub fn from_meta(variant: &'static VariantV2) -> Self {
        Self {
            meta: variant,
            screaming_snake_case_name: str_util::to_snake_case(variant.ty.name).to_uppercase(),
        }
    }
}

struct CSharpField {
    meta: &'static FieldV2,
    // UpperCamelCase
    upper_camel_case_name: String,
    // lowerCamelCase
    lower_camel_case_name: String,
    type_name: String,
}

impl CSharpField {
    #[inline]
    pub fn from_meta(field: &'static FieldV2) -> Self {
        Self {
            meta: field,
            upper_camel_case_name: str_util::to_camel_case(field.name, true),
            lower_camel_case_name: str_util::to_camel_case(field.name, false),
            type_name: get_type(field.ty),
        }
    }
}

fn get_type(ty: &'static FieldTypeV2) -> String {
    match *ty {
        FieldTypeV2::Primitive(name) => match name {
            "u8" => "Byte".to_owned(),
            "u16" => "UInt16".to_owned(),
            "u32" => "UInt32".to_owned(),
            "u64" => "UInt64".to_owned(),
            "i8" => "SByte".to_owned(),
            "i16" => "Int16".to_owned(),
            "i32" => "Int32".to_owned(),
            "i64" => "Int64".to_owned(),
            "bool" => "Boolean".to_owned(),
            _ => name.to_owned(),
        },

        FieldTypeV2::Meta(meta) => match meta {
            MetaV2::Struct(&StructV2 { name, .. }) => name.to_owned(),
            MetaV2::Enum(&EnumV2 { name, .. }) => name.to_owned(),
        },

        FieldTypeV2::MetaRef(name) => name.to_owned(),

        FieldTypeV2::Vec(field_type) => format!("SVector<{}>", get_type(field_type)),
        FieldTypeV2::List(field_type) => format!("SList<{}>", get_type(field_type)),
        FieldTypeV2::Map(field_type) => format!("SDictionary<{}>", get_type(field_type)),
    }
}
