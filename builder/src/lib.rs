use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident, Type, Field, Data};
use syn::ext::IdentExt;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.to_string();
    println!("结构体名称:{:?}", name);
    unimplemented!()
}

struct Fd {
    name: Ident,
    ty: Type,
    optional: bool,
}

struct BuilderContext {
    name: Ident,
    fileds: Vec<Fd>,
}

impl From<Field> for Fd {
    fn from(value: Field) -> Self {
        let (optional,ty) = get_option_inner(&value.ty);
        Self {
            name: value.ident.unwrap(),
            ty: ty.to_owned(),
            optional: optional
        }
    }
}

impl From<DeriveInput> for BuilderContext {
    fn from(value: DeriveInput) -> Self {
        let name = value.ident;
        let fileds = if let Data::Struct(syn::DataStruct {fields:syn::Fields::Named(syn::FieldsNamed{named,..}),..}) = value.data {
            named
        }else{
            panic!("Unsupported data type")
        };
        let fds = fileds.into_iter().map(Fd::from).collect();
        Self {
            name,
            fileds:fds
        }
    }
}

fn get_option_inner(ty: &Type) -> (bool, &Type) {
    if let Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. }) = ty {
        if let Some(v) = segments.iter().next() {
            if(v.ident == "Option") {
                let t = match &v.arguments {
                    syn::PathArguments::AngleBracketed(a) => match a.args.iter().next() {
                        Some(syn::GenericArgument::Type(t)) => t,
                        _ => panic!("Not sure what to do wth other GenericArgument")
                    }
                    _ => panic!("Not sure what to do with other PathArguments")
                };
                return (true,t);
            }
        }
    }
    return (false,ty)
}
