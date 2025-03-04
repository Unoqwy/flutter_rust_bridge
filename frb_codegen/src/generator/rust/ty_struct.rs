use crate::generator::rust::ty::*;
use crate::generator::rust::ExternFuncCollector;
use crate::ir::*;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeStructRefGenerator, IrTypeStructRef);

impl TypeRustGeneratorTrait for TypeStructRefGenerator<'_> {
    fn wire2api_body(&self) -> Option<String> {
        let api_struct = self.ir.get(self.context.ir_file);
        let fields_str = &api_struct
            .fields
            .iter()
            .map(|field| {
                format!(
                    "{} self.{}.wire2api()",
                    if api_struct.is_fields_named {
                        field.name.rust_style().to_string() + ": "
                    } else {
                        String::new()
                    },
                    field.name.rust_style()
                )
            })
            .collect::<Vec<_>>()
            .join(",");

        let (left, right) = api_struct.brackets_pair();
        Some(format!(
            "{}{}{}{}",
            self.ir.rust_api_type(),
            left,
            fields_str,
            right
        ))
    }

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        let s = self.ir.get(self.context.ir_file);
        Some(
            s.fields
                .iter()
                .map(|field| {
                    format!(
                        "{}: {}{}",
                        field.name.rust_style(),
                        field.ty.rust_wire_modifier(),
                        field.ty.rust_wire_type()
                    )
                })
                .collect(),
        )
    }

    fn impl_intodart(&self) -> String {
        let src = self.ir.get(self.context.ir_file);

        let body = src
            .fields
            .iter()
            .map(|field| {
                format!(
                    "self.{}.into_dart()",
                    field.name_rust_style(src.is_fields_named)
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");

        format!(
            "impl support::IntoDart for {} {{
                fn into_dart(self) -> support::DartCObject {{
                    vec![
                        {}
                    ].into_dart()
                }}
            }}
            impl support::IntoDartExceptPrimitive for {} {{}}
            ",
            self.ir.name, body, self.ir.name,
        )
    }

    fn new_with_nullptr(&self, _collector: &mut ExternFuncCollector) -> String {
        let src = self.ir.get(self.context.ir_file);

        let body = {
            src.fields
                .iter()
                .map(|field| {
                    format!(
                        "{}: {},",
                        field.name.rust_style(),
                        if field.ty.rust_wire_is_pointer() {
                            "core::ptr::null_mut()"
                        } else {
                            "Default::default()"
                        }
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        };
        format!(
            r#"impl NewWithNullPtr for {} {{
                    fn new_with_null_ptr() -> Self {{
                        Self {{ {} }}
                    }}
                }}
            "#,
            self.ir.rust_wire_type(),
            body,
        )
    }

    fn imports(&self) -> Option<String> {
        let api_struct = self.ir.get(self.context.ir_file);
        if api_struct.path.is_some() {
            Some(format!(
                "use {};",
                api_struct.path.as_ref().unwrap().join("::")
            ))
        } else {
            None
        }
    }
}
