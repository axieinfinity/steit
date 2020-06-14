use crate::{
    gen::{str_util, Generator, Writer},
    meta::*,
    wire_fmt::WireType,
};

pub struct CSharpSetting {
    namespace: String,
    used_namespaces: Vec<String>,
}

impl CSharpSetting {
    pub fn new(namespace: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            used_namespaces: Vec::new(),
        }
    }

    pub fn using_namespaces(
        mut self,
        used_namespaces: impl IntoIterator<Item = impl ToString>,
    ) -> Self {
        self.used_namespaces = used_namespaces
            .into_iter()
            .map(|namespace| namespace.to_string())
            .collect();

        self
    }
}

pub struct CSharpGenerator;

impl CSharpGenerator {
    pub fn gen_file_opening(&self, setting: &<Self as Generator>::Setting, writer: &mut Writer) {
        writer.writeln("using System;").newline();

        let mut used_namespaces = vec![
            "Steit.Builtins".to_string(),
            "Steit.Codec".to_string(),
            "Steit.Collections".to_string(),
            "Steit.State".to_string(),
            "Steit.State.Event".to_string(),
        ];

        used_namespaces.extend_from_slice(&setting.used_namespaces);
        used_namespaces.sort();

        for used_namespace in used_namespaces {
            writer.writeln(format!("using {};", used_namespace));
        }

        writer
            .newline()
            .writeln(format!("namespace {} {{", &setting.namespace))
            .indent();
    }

    pub fn gen_file_closing(&self, writer: &mut Writer) {
        writer.outdent_writeln("}");
    }
}

impl Generator for CSharpGenerator {
    const INDENT_SIZE: usize = 4;

    type Setting = CSharpSetting;

    fn gen_struct(
        &self,
        r#struct: &StructMeta,
        is_variant: bool,
        setting: &Self::Setting,
        writer: &mut Writer,
    ) {
        let name = r#struct.name.csharp(String::from);
        let var_name = str_util::uncap_first_char(&name);

        let type_name = if !is_variant {
            format!("{}{}", &name, type_params(r#struct.type_params))
        } else {
            name.clone()
        };

        let variant_accessibility = if is_variant { "internal" } else { "public" };
        let fields: Vec<_> = r#struct.fields.iter().map(CSharpField::from_meta).collect();

        if !is_variant {
            self.gen_file_opening(setting, writer);
        }

        writer
            .writeln(format!(
                "public sealed partial class {} : IState {{",
                &type_name
            ))
            .indent();

        writer.writeln("public Path Path { get; }");

        if fields.len() > 1 {
            writer.newline();
        }

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
                // "{} {}(Path? path = null) {{",
                "{} {}(Path path = null) {{",
                variant_accessibility, name,
            ))
            .indent();

        for type_param in r#struct.type_params {
            writer.writeln(format!(
                "StateFactory.ValidateType(typeof({}));",
                type_param,
            ));
        }

        if r#struct.type_params.len() > 1 && fields.len() > 1 {
            writer.newline();
        }

        writer.writeln("this.Path = path ?? Path.Root;");

        if r#struct.type_params.len() > 1 && fields.len() > 1 {
            writer.newline();
        }

        // Initiate nested states
        for field in &fields {
            let init = match field.meta.ty {
                FieldTypeMeta::Type(TypeMeta::Primitive(_, _)) => None,
                FieldTypeMeta::Type(TypeMeta::Ref(_, _)) => Some(format!(
                    "new {}(this.Path.GetNested({}))",
                    field.type_name, field.meta.tag,
                )),
                FieldTypeMeta::TypeParam(type_param) => Some(format!(
                    "StateFactory.Construct<{}>(this.Path.GetNested({}))",
                    type_param, field.meta.tag,
                )),
            };

            if let Some(init) = init {
                writer.writeln(format!("this.{} = {};", field.upper_camel_case_name, init));
            }
        }

        writer.outdent_writeln("}").newline();

        // Declare events
        for field in &fields {
            writer.writeln(format!(
                // "public static event EventHandler<FieldUpdateEventArgs<{}, {}>>? On{}Update;",
                "public static event EventHandler<FieldUpdateEventArgs<{}, {}>> On{}Update;",
                field.type_name, type_name, field.upper_camel_case_name,
            ));
        }

        if !fields.is_empty() {
            writer.newline();
        }

        // Support clearing a field's updated events
        for field in &fields {
            writer.write_indentation().write(format!(
                "public static void Clear{}UpdateHandlers() {{",
                field.upper_camel_case_name,
            ));

            if fields.len() > 1 {
                writer.write(" ");
            } else {
                writer.newline().indent().write_indentation();
            }

            writer.write(format!("On{}Update = null;", field.upper_camel_case_name));

            if fields.len() > 1 {
                writer.write(" ");
            } else {
                writer.newline().outdent().write_indentation();
            }

            writer.write("}").newline();
        }

        if !fields.is_empty() {
            writer.newline();
        }

        // Support clearing all fields' updated events
        writer
            .write_indentation()
            .write("public static void ClearUpdateHandlers() {");

        if !fields.is_empty() {
            writer.indent();
        } else {
            writer.write(" ");
        }

        for field in &fields {
            if !fields.is_empty() {
                writer.newline().write_indentation();
            }

            writer.write(format!("On{}Update = null;", field.upper_camel_case_name));
        }

        if !fields.is_empty() {
            writer.outdent().newline().write_indentation();
        }

        writer
            .write("}")
            .newline()
            .newline()
            .writeln(format!(
                // "{} static {} Deserialize(IReader reader, Path? path = null) {{",
                "{} static {} Deserialize(IReader reader, Path path = null) {{",
                variant_accessibility, type_name,
            ))
            .indent_writeln(format!("var {} = new {}(path);", var_name, type_name))
            .writeln(format!(
                "{}.Replace(reader, shouldNotify: false);",
                var_name
            ))
            .writeln(format!("return {};", var_name))
            .outdent_writeln("}")
            .newline()
            .writeln("public WireType? GetWireType(UInt32 tag) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Return wire types
        for field in r#struct.fields {
            let wire_type = match field.ty {
                FieldTypeMeta::Type(TypeMeta::Primitive(_, wire_type)) => match wire_type {
                    WireType::Varint => "WireType.Varint".to_string(),
                    WireType::Sized => "WireType.Sized".to_string(),
                },

                FieldTypeMeta::Type(TypeMeta::Ref(_, _)) => "WireType.Sized".to_string(),

                FieldTypeMeta::TypeParam(type_param) => format!(
                    "StateFactory.IsStateType(typeof({})) ? WireType.Sized : WireType.Varint",
                    type_param,
                ),
            };

            writer.writeln(format!("case {}: return {};", field.tag, wire_type));
        }

        writer
            .writeln("default: return null;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            // .writeln("public IState? GetNested(UInt32 tag) {")
            .writeln("public IState GetNested(UInt32 tag) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Return nested states
        for field in &fields {
            let nested = match field.meta.ty {
                FieldTypeMeta::Type(TypeMeta::Primitive(_, _)) => None,
                FieldTypeMeta::Type(TypeMeta::Ref(_, _)) => {
                    Some(format!("this.{}", field.upper_camel_case_name))
                }
                FieldTypeMeta::TypeParam(_) => {
                    Some(format!("this.{} as IState", field.upper_camel_case_name))
                }
            };

            if let Some(nested) = nested {
                writer.writeln(format!("case {}: return {};", field.meta.tag, nested));
            }
        }

        writer
            .writeln("default: return null;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln("public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Replace fields and notify event handlers
        for field in &fields {
            match field.meta.ty {
                FieldTypeMeta::Type(TypeMeta::Primitive(_, _)) => {
                    writer.writeln(format!(
                        "case {0}: this.{1} = this.MaybeNotify({0}, reader.Read{2}(), this.{1}, On{1}Update, shouldNotify); break;",
                        field.meta.tag,
                        field.upper_camel_case_name,
                        field.type_name,
                    ));
                }

                FieldTypeMeta::Type(TypeMeta::Ref(_, _)) => {
                    writer.writeln(format!(
                        "case {0}: this.{1} = this.MaybeNotify({0}, {2}.Deserialize(reader, this.Path.GetNested({0})), this.{1}, On{1}Update, shouldNotify); break;",
                        field.meta.tag,
                        field.upper_camel_case_name,
                        field.type_name,
                    ));
                }

                FieldTypeMeta::TypeParam(_) => {
                    writer.writeln(format!(
                        "case {0}: this.{1} = this.MaybeNotify({0}, StateFactory.Deserialize<{2}>(reader, this.Path, {0}), this.{1}, On{1}Update, shouldNotify); break;",
                        field.meta.tag,
                        field.upper_camel_case_name,
                        field.type_name,
                    ));
                }
            };
        }

        writer
            .writeln("default: reader.SkipField(wireType); break;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln(
                "public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }",
            )
            .writeln("public void ReplayListPop() { throw new NotSupportedException(); }")
            .writeln(
                "public void ReplayMapRemove(UInt32 key) { throw new NotSupportedException(); }",
            )
            .newline()
            .writeln("private TValue MaybeNotify<TValue>(")
            .indent_writeln("UInt32 tag,")
            .writeln("TValue newValue,")
            .writeln("TValue oldValue,")
            .writeln(format!(
                // "EventHandler<FieldUpdateEventArgs<TValue, {}>>? handler,",
                "EventHandler<FieldUpdateEventArgs<TValue, {}>> handler,",
                type_name
            ))
            .writeln("bool shouldNotify")
            .outdent_writeln(") {")
            .indent_writeln("if (shouldNotify) {")
            .indent_writeln(format!(
                "var args = new FieldUpdateEventArgs<TValue, {}>(tag, newValue, oldValue, this);",
                type_name
            ))
            .writeln("handler?.Invoke(this, args);")
            .outdent_writeln("}")
            .newline()
            .writeln("return newValue;")
            .outdent_writeln("}")
            .outdent_writeln("}");

        if !is_variant {
            self.gen_file_closing(writer);
        }
    }

    fn gen_enum(&self, r#enum: &EnumMeta, setting: &Self::Setting, writer: &mut Writer) {
        let name = r#enum.name.csharp(String::from);
        let var_name = str_util::uncap_first_char(&name);
        let type_name = format!("{}{}", &name, type_params(r#enum.type_params));

        let variants: Vec<_> = r#enum
            .variants
            .iter()
            .map(CSharpVariant::from_meta)
            .collect();

        let default_variant = variants
            .iter()
            .find(|variant| variant.meta.default())
            .unwrap_or_else(|| panic!("expected a default variant for enum {}", name));

        self.gen_file_opening(setting, writer);

        writer
            .writeln(format!(
                "public sealed partial class {} : IEnumState {{",
                type_name
            ))
            .indent();

        // Declare variant tag constants
        for variant in &variants {
            writer.writeln(format!(
                "public const UInt32 {}Tag = {};",
                variant.upper_camel_case_name, variant.meta.tag,
            ));
        }

        writer
            .newline()
            .writeln("public Path Path { get; }")
            .newline()
            .writeln("public UInt32 Tag { get; private set; }")
            .writeln("public IState Variant { get; private set; }")
            .newline();

        // Return variant values
        for variant in r#enum.variants {
            writer.writeln(format!(
                // "public {0}? {0}Variant {{ get {{ return this.Variant as {0}; }} }}",
                "public {0} {0}Variant {{ get {{ return this.Variant as {0}; }} }}",
                variant.ty.name.csharp(String::from),
            ));
        }

        writer
            .newline()
            // .writeln(format!("public {}(Path? path = null) {{", name))
            .writeln(format!("public {}(Path path = null) {{", name))
            .indent();

        for type_param in r#enum.type_params {
            writer.writeln(format!(
                "StateFactory.ValidateType(typeof({}));",
                type_param,
            ));
        }

        if r#enum.type_params.len() > 1 {
            writer.newline();
        }

        writer.writeln("this.Path = path ?? Path.Root;");

        if r#enum.type_params.len() > 1 {
            writer.newline();
        }

        writer
            .writeln(format!("this.Tag = {};", default_variant.meta.tag))
            .writeln(format!(
                "this.Variant = new {}(this.Path.GetNested({}));",
                default_variant.meta.ty.name.csharp(String::from),
                default_variant.meta.tag,
            ))
            .outdent_writeln("}")
            .newline()
            .writeln(format!(
                // "public static event EventHandler<VariantUpdateEventArgs<{}>>? OnUpdate;",
                "public static event EventHandler<VariantUpdateEventArgs<{}>> OnUpdate;",
                type_name,
            ))
            .newline()
            .writeln("public static void ClearUpdateHandlers() {")
            .indent_writeln("OnUpdate = null;")
            .outdent_writeln("}")
            .newline()
            .writeln(format!(
                // "public static {} Deserialize(IReader reader, Path? path = null) {{",
                "public static {} Deserialize(IReader reader, Path path = null) {{",
                type_name,
            ))
            .indent_writeln(format!("var {} = new {}(path);", var_name, type_name))
            .writeln(format!(
                "{}.Replace(reader, shouldNotify: false);",
                var_name
            ))
            .writeln(format!("return {};", var_name))
            .outdent_writeln("}")
            .newline()
            .writeln("public WireType? GetWireType(UInt32 tag) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Return wire types
        for variant in r#enum.variants {
            writer.writeln(format!("case {}: return WireType.Sized;", variant.tag));
        }

        writer
            .writeln("default: return null;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            // .writeln("public IState? GetNested(UInt32 tag) {")
            .writeln("public IState GetNested(UInt32 tag) {")
            .indent_writeln("return tag == this.Tag ? this.Variant : null;")
            .outdent_writeln("}")
            .newline()
            .writeln("public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Replace fields and notify event handlers
        for variant in r#enum.variants {
            writer.writeln(format!(
                "case {0}: this.UpdateAndNotify({0}, {1}.Deserialize(reader, this.Path.GetNested({0})), shouldNotify); break;",
                variant.tag, variant.ty.name.csharp(String::from),
            ));
        }

        writer
            .writeln("default: reader.SkipToEnd(); break;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln("public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }")
            .writeln("public void ReplayListPop() { throw new NotSupportedException(); }")
            .writeln("public void ReplayMapRemove(UInt32 key) { throw new NotSupportedException(); }")
            .newline()
            .writeln("private void UpdateAndNotify(UInt32 newTag, IState newVariant, bool shouldNotify) {")
            .indent_writeln("if (shouldNotify) {")
            .indent_writeln(format!(
                "var args = new VariantUpdateEventArgs<{}>(newTag, newVariant, this.Tag, this.Variant, this);",
                type_name,
            ))
            .writeln(format!("{}.OnUpdate?.Invoke(this, args);", type_name))
            .outdent_writeln("}")
            .newline()
            .writeln("this.Tag = newTag;")
            .writeln("this.Variant = newVariant;")
            .outdent_writeln("}");

        for variant in r#enum.variants {
            writer
                .newline()
                .writeln(format!(
                    "// Variant ({}): {}",
                    variant.tag,
                    variant.ty.name.csharp(String::from),
                ))
                .newline();

            self.gen_struct(&variant.ty, true, setting, writer);
        }

        writer.outdent_writeln("}");

        self.gen_file_closing(writer);
    }
}

struct CSharpField {
    meta: &'static FieldMeta,
    // UpperCamelCase
    upper_camel_case_name: String,
    type_name: String,
}

impl CSharpField {
    pub fn from_meta(field: &'static FieldMeta) -> Self {
        Self {
            meta: field,
            upper_camel_case_name: field
                .name
                .csharp(|name| str_util::to_camel_case(name, true)),
            type_name: field_type(field.ty),
        }
    }
}

struct CSharpVariant {
    meta: &'static VariantMeta,
    // UpperCamelCase
    upper_camel_case_name: String,
}

impl CSharpVariant {
    pub fn from_meta(variant: &'static VariantMeta) -> Self {
        Self {
            meta: variant,
            upper_camel_case_name: variant
                .ty
                .name
                .csharp(|name| str_util::to_camel_case(name, true)),
        }
    }
}

fn type_params(type_params: &'static [&'static str]) -> String {
    if type_params.is_empty() {
        return "".to_string();
    }

    format!("<{}>", type_params.join(", "))
}

fn field_type(ty: &'static FieldTypeMeta) -> String {
    match *ty {
        FieldTypeMeta::Type(ty) => match ty {
            TypeMeta::Primitive(name, _) => name
                .csharp
                .expect("expected a C# name for every primitive type")
                .to_string(),

            TypeMeta::Ref(name, type_args) => {
                let type_name = name.csharp(String::from);

                if type_args.is_empty() {
                    return type_name;
                }

                let mut type_args: Vec<_> = type_args.iter().map(field_type).collect();

                // A hack to bypass `Box`
                if &type_name == "Box" {
                    return type_args[0].clone();
                }

                // A hack to shadow the first type argument of `Map`
                if &type_name == "StateMap" {
                    type_args.remove(0);
                }

                format!("{}<{}>", type_name, type_args.join(", "))
            }
        },

        FieldTypeMeta::TypeParam(type_param) => type_param.to_string(),
    }
}
