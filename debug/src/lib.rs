use proc_macro2::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, Data, DataStruct, Fields, FieldsNamed, Token};
use syn::punctuated::Punctuated;

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match do_expand(&input) {
        Ok(ret) => ret.into(),
        Err(e) => e.to_compile_error().into()
    }
}

fn do_expand(input: &DeriveInput) -> syn::Result<TokenStream> {
    let ret = generate_debug_trait(&input)?;
    Ok(ret)
}

type StructFields = Punctuated<Field,Token![,]>;

fn get_fields_from_derive_input(d: &DeriveInput) -> syn::Result<&StructFields> {
    if let Data::Struct(DataStruct{fields:Fields::Named(FieldsNamed{ref named,..}),..}) = d.data {
        return Ok(named)
    }else {
        Err(syn::Error::new_spanned(d,"Must define on a Struct,not enum".to_string()))
    }
}


// 自定义格式化输出需要生成fmt.debug_struct等模式代码
// impl fmt::Debug for Foo {
//     fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
//         fmt.debug_struct("Foo")
//             .field("bar", &self.bar)
//             .field("baz", &self.baz)
//             .finish()
//     }
// }

fn generate_debug_trait(st: &DeriveInput) -> syn::Result<TokenStream> {
    let fields = get_fields_from_derive_input(st)?;
    let struct_name_ident = &st.ident;
    let struct_name_literal = struct_name_ident.to_string();
    //构造一个空的TokenStream,extend相当于拼接几个TokenStream为一个完整的TokenStream
    let mut token_stream = TokenStream::new();
    token_stream.extend(quote! {
        fmt.debug_struct(#struct_name_literal)
    });
    for field in fields.iter() {
        let field_name_ident = field.ident.as_ref().unwrap();
        let field_name_literal = field_name_ident.to_string();
        token_stream.extend(quote! {
            .field(#field_name_literal,&self.#field_name_ident)
        });
    }
    token_stream.extend(quote! {
        .finish()
    });

    let token_stream_result = quote! {
        impl std::fmt::Debug for #struct_name_ident {
            fn fmt(&self,fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #token_stream
            }
        }
    };
    Ok(token_stream_result)
}

fn get_field_attr(field: &Field) -> syn::Result<Option<String>> {
    unimplemented!()
}

