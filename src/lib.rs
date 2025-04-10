use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, DeriveInput, ItemStatic, Result, TypePath,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct SingletonArgs {
    type_name: TypePath,
}

impl Parse for SingletonArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let type_name = input.parse()?;
        Ok(SingletonArgs { type_name })
    }
}

/// Create a singleton from a static variable.
///
/// # Examples
/// ```
/// use std::sync::LazyLock;
/// use uuid::Uuid;
/// use proc_singleton::singleton_from_static;
///
/// #[singleton_from_static(Identifier)]
/// static IDENT: LazyLock<Identifier> = LazyLock::new(|| {
///     Identifier {
///         id: Uuid::new_v4(),
///     }
/// });
///
/// struct Identifier {
///     id: Uuid,
/// }
///
/// fn main() {
///     let instance = Identifier::get_instance();
///     let ptr = instance as *const Identifier;
///     let same_ptr = Identifier::get_instance() as *const Identifier;
///
///     assert_eq!(ptr, same_ptr);
/// }
/// ```
#[proc_macro_attribute]
pub fn singleton_from_static(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as SingletonArgs);
    let type_name = &args.type_name;
    let input = parse_macro_input!(item as ItemStatic);
    let static_name = &input.ident;

    let expanded = quote! {
        #input

        impl #type_name {
            pub fn get_instance() -> &'static #type_name {
                &#static_name
            }
        }
    };

    expanded.into()
}

/// Derives a singleton implementation for a struct based on a static variable.
///
/// # Examples
/// ```
/// use std::sync::LazyLock;
/// use uuid::Uuid;
/// use proc_singleton::Singleton;
///
/// static IDENT: LazyLock<Identifier> = LazyLock::new(|| {
///     Identifier {
///         id: Uuid::new_v4(),
///     }
/// });
/// #[derive(Singleton)]
/// #[singleton(IDENT)]
/// struct Identifier {
///     id: Uuid,
/// }
///
/// fn main() {
///     let instance = Identifier::get_instance();
///     let ptr = instance as *const Identifier;
///     let same_ptr = Identifier::get_instance() as *const Identifier;
///
///     assert_eq!(ptr, same_ptr);
/// }
/// ```
#[proc_macro_derive(Singleton, attributes(singleton))]
pub fn derive_singleton(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let static_name =
        find_singleton_static_name(&input.attrs).expect("#[singleton(STATIC_NAME)] is required");

    let expanded = quote! {
        impl #struct_name {
            pub fn get_instance() -> &'static #struct_name {
                &#static_name
            }
        }
    };

    expanded.into()
}

fn find_singleton_static_name(attrs: &[Attribute]) -> Option<syn::Ident> {
    for attr in attrs {
        if attr.path().is_ident("singleton") {
            if let Ok(expr) = attr.parse_args::<syn::Expr>() {
                if let syn::Expr::Path(expr_path) = expr {
                    if let Some(ident) = expr_path.path.get_ident() {
                        return Some(ident.clone());
                    }
                }
            }
        }
    }
    None
}
