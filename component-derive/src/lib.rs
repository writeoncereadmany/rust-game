use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Constant)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl entity::Component for #ident {
            fn get(entity: &entity::Entity) -> Option<Self> {
                Some(entity.get::<#ident>()?.clone())
            }
        }
    };
    output.into()
}

#[proc_macro_derive(Variable)]
pub fn derive_variable(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl entity::Component for #ident {
            fn get(entity: &entity::Entity) -> Option<Self> {
                Some(entity.get::<#ident>()?.clone())
            } 
        }
        impl entity::Variable for #ident {
            fn set(self, entity: &mut entity::Entity) {
                entity.set(self)
            }
            fn remove(entity: &mut entity::Entity) {
                entity.remove::<#ident>()
            }
        }
    };
    output.into()
}

#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl EventTrait for #ident {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn dispatch(&self, dispatcher: &engine::events::Dispatcher, world: &mut entity::Entities, events: &mut engine::events::Events) {
                dispatcher.dispatch(self, world, events);
            }
        }
    };
    output.into()
}