use darling::FromField;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type, Field, Data, DataStruct, Fields, FieldsNamed, TypePath, Path, GenericArgument, Generics, GenericParam, parse_quote};

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // let context = BuilderContext::from(input);
    // let name = &context.name;
    let name = input.ident;
    //fmt.field方法中需要结构体每个字段实现Debug trait,所以第一步需要给结构体每个字段实现Debug trait
    let generics = add_trait_bounds(input.generics);
    // let token = quote! {
    //     impl fmt::Debug for #name {
    //          fn fmt(&self,fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    //
    //             fmt.debug_struct(#name)
    //                .field("bar", &self.bar)
    //                .field("baz", &self.baz)
    //                .finish()
    //         }
    //     }
    // };
    // token.into()
    unimplemented!()
}

fn add_trait_bounds(mut generics: Generics) -> Generics{
     for param in &mut generics {
         if let GenericParam::Type(ref mut typeparam) = *param {
             typeparam.bounds.push(parse_quote!(fmt::Debug))
         }
     }
    generics
}

struct Fd {
    name: Ident,
    ty: Type,
    optional: bool,
    opts: Opts
}

#[derive(Debug,Default,FromField)]
#[darling(default,attributes(debug))]
struct Opts {
    each: Option<String>,
    default: Option<String>
}

/// 我们需要的描述一个 struct 的所有信息
struct BuilderContext {
    name: Ident,
    fields: Vec<Fd>,
}

/// 把一个 Field 转换成 Fd
impl From<Field> for Fd {
    fn from(f: Field) -> Self {
        let (optional, ty) = get_option_inner(&f.ty);
        let opts = Opts::from_field(&f).unwrap_or_default();
        Self {
            // 此时，我们拿到的是 NamedFields，所以 ident 必然存在
            name: f.ident.unwrap(),
            optional,
            ty: ty.to_owned(),
            opts
        }
    }
}

/// 把 DeriveInput 转换成 BuilderContext
impl From<DeriveInput> for BuilderContext {
    fn from(input: DeriveInput) -> Self {
        let name = input.ident;
        let fields = if let Data::Struct(DataStruct {
                                             fields: Fields::Named(FieldsNamed { named, .. }),
                                             ..
                                         }) = input.data
        {
            named
        } else {
            panic!("Unsupported data type");
        };

        let fds = fields.into_iter().map(Fd::from).collect();
        Self { name, fields: fds }
    }
}

// 如果是 T = Option<Inner>，返回 (true, Inner)；否则返回 (false, T)
// 如果是 T = Option<Inner>，返回 (true, Inner)；否则返回 (false, T)
fn get_option_inner(ty: &Type) -> (bool, &Type) {
    get_type_inner(ty, "Option")
}

// 如果是 T = Vec<Inner>，返回 (true, Inner)；否则返回 (false, T)
fn get_vec_inner(ty: &Type) -> (bool, &Type) {
    get_type_inner(ty, "Vec")
}

fn get_type_inner<'a>(ty: &'a Type, name: &str) -> (bool, &'a Type) {
    // 首先模式匹配出 segments
    if let Type::Path(TypePath {
                          path: Path { segments, .. },
                          ..
                      }) = ty
    {
        if let Some(v) = segments.iter().next() {
            if v.ident == name {
                // 如果 PathSegment 第一个是 Option/Vec 等类型，那么它内部应该是 AngleBracketed，比如 <T>
                // 获取其第一个值，如果是 GenericArgument::Type，则返回
                let t = match &v.arguments {
                    syn::PathArguments::AngleBracketed(a) => match a.args.iter().next() {
                        Some(GenericArgument::Type(t)) => t,
                        _ => panic!("Not sure what to do with other GenericArgument"),
                    },
                    _ => panic!("Not sure what to do with other PathArguments"),
                };
                return (true, t);
            }
        }
    }
    (false, ty)
}
