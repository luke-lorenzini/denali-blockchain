use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput, Expr, Meta, punctuated::Punctuated, Token, parse::Parser,
};

#[proc_macro_attribute]
pub fn generate_create_thing(attr: TokenStream, item: TokenStream) -> TokenStream {
    let meta = parse_macro_input!(attr as Meta);
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    let args: Punctuated<Expr, Token![,]> = if let Meta::List(meta_list) = meta {
        if meta_list.path.is_ident("args") {
                let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
                parser.parse2(meta_list.tokens).expect("Could not parse args(...) expressions")
        } else {
            panic!("expected #[generate_code(args(...))]")
        }
    } else {
        panic!("expected #[generate_code(args(...))]")
    };

    let output = quote! {
        #input
        use std::ffi::c_void;
        use denali::plugins::RawTraitObject;

        #[unsafe(no_mangle)]
        pub extern "C" fn create_thing() 
        -> *mut c_void 
        {
            let boxed: Box<dyn Thing> = Box::new(#struct_name::new(#args));
            let raw_fat_ptr = Box::into_raw(boxed);
            unsafe {
                let (data_ptr, vtable_ptr): (*mut c_void, *mut c_void) = std::mem::transmute(raw_fat_ptr);

                let boxed_raw_trait_object = Box::new(RawTraitObject {
                    data_ptr,
                    vtable_ptr,
                });
                Box::into_raw(boxed_raw_trait_object).cast::<c_void>()
            }
        }
    };
    output.into()
}
