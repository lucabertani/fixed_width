// mod any_value;

use std::str::FromStr;

use darling::{ast, util, FromDeriveInput, FromField};
use proc_macro::TokenStream;
//use darling::FromDeriveInput;
use quote::quote;
use strum::EnumString;
use syn::{parse_macro_input, DeriveInput, Ident, Type};

/*#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(fixed_width), forward_attrs(allow, doc, cfg))]
struct Opts {
    size: u32,
}*/

// cargo expand --test test_simple
// RUSTFLAGS="-Z macro-backtrace" cargo test
// da eseguire con rust nightly

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(fixed_width), supports(struct_named))]
struct FixedWidthFields {
    ident: Ident,
    data: ast::Data<util::Ignored, FixedWidthField>,
}

#[derive(Debug, FromField)]
#[darling(attributes(fixed_width))]
struct FixedWidthField {
    ident: Option<Ident>,
    ty: Type,
    //#[darling(default)]
    //skip: bool,
    #[darling(default)]
    size: u32,
    pad: Option<char>,
    #[darling(default)]
    pad_left: bool,
}

impl FixedWidthField {
    fn field_name(&self) -> String {
        self.ident()
            .and_then(|i| Some(i.to_string()))
            .unwrap_or(String::new())
    }

    fn field_type(&self) -> FieldType {
        if let Type::Path(path) = self.ty() {
            let field_type = &path.path.segments.first().unwrap().ident;
            let field_type_enum = FieldType::from_str(field_type.to_string().as_str()).expect(
                format!("Unable to parse {} into FieldType", field_type.to_string()).as_str(),
            );
            field_type_enum
        } else {
            panic!("Unexpected type: {:?}", self.ty());
        }
    }

    fn ident(&self) -> Option<&Ident> {
        self.ident.as_ref()
    }

    fn ty(&self) -> &Type {
        &self.ty
    }

    fn pad(&self) -> Option<char> {
        self.pad
    }

    fn with_pad(&self) -> u8 {
        let pad = self.pad.unwrap_or(' ') as u8;
        pad
    }

    fn pad_left(&self) -> bool {
        self.pad_left
    }

    fn size(&self) -> usize {
        self.size as usize
    }
}

#[derive(Debug, Clone, Copy, EnumString, strum::Display)]
enum FieldType {
    String,
}

#[proc_macro_derive(FixedWidth, attributes(fixed_width))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let fw: FixedWidthFields = FixedWidthFields::from_derive_input(&input).unwrap();
    //println!("Derive {:#?}", fw);
    //let DeriveInput { ident, .. } = input;
    //println!("Derive {:#?}", input);
    let ident = input.ident;
    /*if let syn::Data::Struct(obj) = input.data {
        let fields = obj.fields;
        for field in fields.iter() {
            let opts = Opts::from_derive_input(field.into());
            println!("field attributes: {:#?}", field.attrs);
        }
    }*/

    let mut fields = Vec::new();

    for field in fw.data.take_struct().unwrap() {
        // println!("field: {:?}", field.ident);
        /*let field_name = field.ident.unwrap();
        if let Type::Path(path) = &field.ty {
            let field_type = &path.path.segments.first().unwrap().ident;
            println!("Field {} of type {}", field_name, field_type);
        }
        let field_type = field.ty;*/

        let field_name = field.field_name();
        //let field_type = field.field_type();

        //println!("Field {} with type {}", field_name, field_type);

        let field_name: proc_macro2::TokenStream = field_name.parse().unwrap();
        /* let pad = field.with_pad();
        let pad_left = field.pad_left();
        let size = field.size();*/
        /*let pad: proc_macro2::TokenStream = proc_macro2::TokenStream::from(field.with_pad());
        let pad_left: proc_macro2::TokenStream = field.pad_left().parse().unwrap();
        let size: proc_macro2::TokenStream = field.size().parse().unwrap();*/
        let pad = field.with_pad();
        let pad_left = field.pad_left();
        let size = field.size();

        /*match field_type {
            FieldType::String => {
                //s.push_str(self.#field_name.to_string().as_ref());
                let convert = quote! {
                    let mut v = fixed_record_length::pad(&self.#field_name, #pad, #pad_left, #size)?;
                    res.append(&mut v);
                };
                fields.push(convert);
            }
        }*/

        let convert = quote! {
            let mut v = fixed_width::pad(&self.#field_name, #pad, #pad_left, #size)?;
            res.append(&mut v);
        };
        fields.push(convert);

        /*let convert = quote! {
            s.push_str(self.#field_name.to_string().as_ref());
        };

        // println!("convert: {:?}", convert);

        fields.push(convert);*/
    }

    let output: proc_macro2::TokenStream = quote! {
        impl FixedWidth for #ident {
            fn to_bytes(&self) -> Result<Vec<u8>, fixed_width::error::FixedWidthError> {
                let mut s = String::new();
                let mut res: Vec<u8> = Vec::new();
                #(#fields)*
                Ok(res)
                //Ok(String::from_utf8(res).unwrap())
                //Ok(String::new)
                //"Hello World!".to_string()
            }
        }
    };

    output.into()

    /*let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;

    let answer = match opts.answer {
        Some(x) => quote! {
            fn answer() -> i32 {
                #x
            }
        },
        None => quote! {},
    };

    let output = quote! {
        impl MyTrait for #ident {
            #answer
        }
    };
    output.into()*/
}

/*use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(my_trait), forward_attrs(allow, doc, cfg))]
struct Opts {
    answer: Option<i32>,
}

#[proc_macro_derive(FixedWidth, attributes(my_trait))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;

    let answer = match opts.answer {
        Some(x) => quote! {
            fn answer() -> i32 {
                #x
            }
        },
        None => quote! {},
    };

    let output = quote! {
        impl MyTrait for #ident {
            #answer
        }
    };
    output.into()
}
*/
