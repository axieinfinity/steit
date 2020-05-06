use crate::{
    gen::{str_util, GeneratorV2, Writer},
    meta::*,
};

pub struct CSharpSetting {
    namespace: String,
}

impl CSharpSetting {
    #[inline]
    pub fn new(namespace: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
        }
    }
}

pub struct CSharpGeneratorV2;

impl CSharpGeneratorV2 {
    pub fn gen_file_opening(&self, setting: &<Self as GeneratorV2>::Setting, writer: &mut Writer) {
        writer
            .writeln("using System;")
            .newline()
            .writeln("using Steit.Builtins;")
            .writeln("using Steit.Codec;")
            .writeln("using Steit.Collections;")
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
            .writeln(format!("public sealed class {} : IState {{", &type_name))
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
                "{} {}(Path? path = null) {{",
                variant_accessibility, name,
            ))
            .indent();

        for type_param in r#struct.type_params {
            writer.writeln(format!(
                "Typing.CheckPrimitiveOrStateType(typeof({}));",
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
            if let FieldTypeMeta::Type(TypeMeta::Primitive(_)) = field.meta.ty {
            } else {
                writer.writeln(format!(
                    "this.{} = Typing.New<{}>(this.Path, {});",
                    field.upper_camel_case_name, field.type_name, field.meta.tag,
                ));
            }
        }

        writer.outdent_writeln("}").newline();

        // Declare events
        for field in &fields {
            writer.writeln(format!(
                "public static event EventHandler<FieldUpdateEventArgs<{}, {}>>? On{}Update;",
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

            writer.write(format!(
                "{}.On{}Update = null;",
                type_name, field.upper_camel_case_name,
            ));

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

            writer.write(format!(
                "{}.On{}Update = null;",
                type_name, field.upper_camel_case_name,
            ));
        }

        if !fields.is_empty() {
            writer.outdent().newline().write_indentation();
        }

        writer
            .write("}")
            .newline()
            .newline()
            .writeln(format!(
                "{} static {} Deserialize(IReader reader, Path? path = null) {{",
                variant_accessibility, type_name,
            ))
            .indent_writeln(format!("var {} = new {}(path);", var_name, type_name))
            .writeln(format!("{}.Replace(reader);", var_name))
            .writeln(format!("return {};", var_name))
            .outdent_writeln("}")
            .newline()
            .writeln("public WireType? GetWireType(UInt32 tag) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Return wire types
        for field in r#struct.fields {
            let wire_type = match field.ty {
                FieldTypeMeta::Type(TypeMeta::Primitive(_)) => "WireType.Varint".to_string(),
                FieldTypeMeta::Type(TypeMeta::Ref(_, _)) => "WireType.Sized".to_string(),
                FieldTypeMeta::TypeParam(type_param) => format!(
                    "Typing.IsStateType(typeof({})) ? WireType.Sized : WireType.Varint",
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
            .writeln("public IState? GetNested(UInt32 tag) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Return nested states
        for field in &fields {
            let nested = match field.meta.ty {
                FieldTypeMeta::Type(TypeMeta::Primitive(_)) => None,
                FieldTypeMeta::Type(TypeMeta::Ref(_, _)) => Some(format!(
                    "case {}: return this.{};",
                    field.meta.tag, field.upper_camel_case_name,
                )),
                FieldTypeMeta::TypeParam(type_param) => Some(format!(
                    "case {}: return Typing.IsStateType(typeof({})) ? this.{} as IState : null;",
                    field.meta.tag, type_param, field.upper_camel_case_name,
                )),
            };

            if let Some(nested) = nested {
                writer.writeln(nested);
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
                FieldTypeMeta::Type(TypeMeta::Primitive(_)) => {
                    writer.writeln(format!(
                        "case {0}: this.{1} = this.MaybeNotify({0}, reader.Read{2}(), this.{1}, {3}.On{1}Update, shouldNotify); break;",
                        field.meta.tag,
                        field.upper_camel_case_name,
                        field.type_name,
                        type_name,
                    ));
                }

                FieldTypeMeta::Type(TypeMeta::Ref(_, _)) => {
                    writer.writeln(format!(
                        "case {0}: this.{1} = this.MaybeNotify({0}, {2}.Deserialize(reader.GetNested(), this.Path.GetNested({0})), this.{1}, {3}.On{1}Update, shouldNotify); break;",
                        field.meta.tag,
                        field.upper_camel_case_name,
                        field.type_name,
                        type_name,
                    ));
                }

                FieldTypeMeta::TypeParam(type_param) => {
                    writer.writeln(format!(
                        "case {0}: this.{1} = this.MaybeNotify({0}, reader.ReadValue<{2}>(this.Path, {0}), this.{1}, {3}.On{1}Update, shouldNotify); break;",
                        field.meta.tag,
                        field.upper_camel_case_name,
                        field.type_name,
                        type_name,
                    ));
                }
            };
        }

        writer
            .writeln("default: reader.SkipField(wireType); break;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln("public bool IsList() { return false; }")
            .writeln("public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }")
            .writeln("public void ReplayListPop() { throw new NotSupportedException(); }")
            .newline()
            .writeln("public bool IsMap() { return false; }")
            .writeln("public void ReplayMapInsert(IReader reader) { throw new NotSupportedException(); }")
            .writeln("public void ReplayMapRemove(IReader reader) { throw new NotSupportedException(); }")
            .newline()
            .writeln("private TValue MaybeNotify<TValue>(")
            .indent_writeln("UInt32 tag,")
            .writeln("TValue newValue,")
            .writeln("TValue oldValue,")
            .writeln(format!("EventHandler<FieldUpdateEventArgs<TValue, {}>>? handler,", type_name))
            .writeln("bool shouldNotify")
            .outdent_writeln(") {")
            .indent_writeln("if (shouldNotify) {")
            .indent_writeln(format!("var args = new FieldUpdateEventArgs<TValue, {}>(tag, newValue, oldValue, this);", type_name))
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
            .find(|variant| variant.meta.default)
            .expect(&format!("expected a default variant for enum {}", name));

        self.gen_file_opening(setting, writer);

        writer
            .writeln(format!("public sealed class {} : IEnumState {{", type_name))
            .indent();

        // Declare variant tag constants
        for variant in &variants {
            writer.writeln(format!(
                "public const UInt32 {}_TAG = {};",
                variant.screaming_snake_case_name, variant.meta.tag,
            ));
        }

        writer
            .newline()
            .writeln("public Path Path { get; }")
            .newline()
            .writeln("public UInt32 VariantTag { get; private set; }")
            .writeln("public IState Variant { get; private set; }")
            .newline();

        // Return variant values
        for variant in r#enum.variants {
            writer.writeln(format!(
                "public {0}? {0}Variant {{ get {{ return this.VariantTag == {1} ? ({0}) this.Variant : null; }} }}",
                variant.ty.name.csharp(String::from), variant.tag,
            ));
        }

        writer
            .newline()
            .writeln(format!("public {}(Path? path = null) {{", name))
            .indent();

        for type_param in r#enum.type_params {
            writer.writeln(format!(
                "Typing.CheckPrimitiveOrStateType(typeof({}));",
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
            .writeln(format!("this.VariantTag = {};", default_variant.meta.tag))
            .writeln(format!(
                "this.Variant = new {}(this.Path.GetNested({}));",
                default_variant.meta.ty.name.csharp(String::from),
                default_variant.meta.tag,
            ))
            .outdent_writeln("}")
            .newline()
            .writeln(format!(
                "public static event EventHandler<VariantUpdateEventArgs<{}>>? OnUpdate;",
                type_name,
            ))
            .newline()
            .writeln("public static void ClearUpdateHandlers() {")
            .indent_writeln(format!("{}.OnUpdate = null;", type_name))
            .outdent_writeln("}")
            .newline()
            .writeln(format!(
                "public static {} Deserialize(IReader reader, Path? path = null) {{",
                type_name,
            ))
            .indent_writeln(format!("var {} = new {}(path);", var_name, type_name))
            .writeln(format!("{}.Replace(reader);", var_name))
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
            .writeln("public IState? GetNested(UInt32 tag) {")
            .indent_writeln("return tag == this.VariantTag ? this.Variant : null;")
            .outdent_writeln("}")
            .newline()
            .writeln("public void ReplaceAt(UInt32 tag, WireType wireType, IReader reader, bool shouldNotify) {")
            .indent_writeln("switch (tag) {")
            .indent();

        // Replace fields and notify event handlers
        for variant in r#enum.variants {
            writer.writeln(format!(
                "case {0}: this.NotifyAndUpdate({0}, {1}.Deserialize(reader, this.Path.GetNested({0})), shouldNotify); break;",
                variant.tag, variant.ty.name.csharp(String::from),
            ));
        }

        writer
            .writeln("default: reader.SkipToEnd(); break;")
            .outdent_writeln("}")
            .outdent_writeln("}")
            .newline()
            .writeln("public bool IsList() { return false; }")
            .writeln("public void ReplayListPush(IReader reader) { throw new NotSupportedException(); }")
            .writeln("public void ReplayListPop() { throw new NotSupportedException(); }")
            .newline()
            .writeln("public bool IsMap() { return false; }")
            .writeln("public void ReplayMapInsert(IReader reader) { throw new NotSupportedException(); }")
            .writeln("public void ReplayMapRemove(IReader reader) { throw new NotSupportedException(); }")
            .newline()
            .writeln("private void NotifyAndUpdate(UInt32 newVariantTag, IState newVariant, bool shouldNotify) {")
            .indent_writeln("if (shouldNotify) {")
            .indent_writeln(format!(
                "var args = new VariantUpdateEventArgs<{}>(newVariantTag, newVariant, this.VariantTag, this.Variant, this);",
                type_name,
            ))
            .writeln(format!("{}.OnUpdate?.Invoke(this, args);", type_name))
            .outdent_writeln("}")
            .newline()
            .writeln("this.VariantTag = newVariantTag;")
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

struct CSharpVariant {
    meta: &'static VariantMeta,
    // SCREAMING_SNAKE_CASE
    screaming_snake_case_name: String,
}

impl CSharpVariant {
    #[inline]
    pub fn from_meta(variant: &'static VariantMeta) -> Self {
        Self {
            meta: variant,
            screaming_snake_case_name: variant
                .ty
                .name
                .csharp(|name| str_util::to_snake_case(name).to_uppercase()),
        }
    }
}

struct CSharpField {
    meta: &'static FieldMeta,
    // UpperCamelCase
    upper_camel_case_name: String,
    type_name: String,
}

impl CSharpField {
    #[inline]
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

fn type_params(type_params: &'static [&'static str]) -> String {
    if type_params.is_empty() {
        return "".to_string();
    }

    format!("<{}>", type_params.join(", "))
}

fn field_type(ty: &'static FieldTypeMeta) -> String {
    match *ty {
        FieldTypeMeta::Type(ty) => match ty {
            TypeMeta::Primitive(name) => name
                .csharp
                .expect("expected a C# name for every primitive type")
                .to_string(),

            TypeMeta::Ref(name, type_args) => {
                let type_name = name.csharp(String::from);

                if type_args.is_empty() {
                    return type_name;
                }

                let type_args: Vec<_> = type_args.iter().map(field_type).collect();

                format!("{}<{}>", type_name, type_args.join(", "))
            }
        },

        FieldTypeMeta::TypeParam(type_param) => type_param.to_string(),
    }
}