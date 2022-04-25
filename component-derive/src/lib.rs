use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl <'a> GetComponent<'a, &'a #ident> for &'a #ident {
            fn get_component(entity: &'a Entity) -> Option<&'a #ident> {
                entity.get_atom()
            }
        }

        impl SetComponent for #ident {
            fn set_on_entity(self, entity: &mut Entity) {
                entity.set_atom(self)
            }
        }
    };
    output.into()
}