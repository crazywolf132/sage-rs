use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive macro for the Component trait.
///
/// This automatically implements the Component trait for a struct,
/// assuming it has the following methods defined:
/// - `update(&mut self, msg: Msg) -> Option<Command<Msg>>`
/// - `view(&self, area: Size) -> String`
///
/// It will also use `handle_key_event` if it exists, otherwise it will use the default implementation.
#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name  = &input.ident;
    let gen = quote! {
        impl Component for #name {
            type Msg = Msg;
            fn update(&mut self, msg: Self::Msg) -> Option<Command<Self::Msg>> { self.update(msg) }
            fn view(&self, area: Size) -> String { self.view(area) }
            fn handle_key_event(&self, key: KeyEvent) -> Option<Self::Msg> { self.handle_key_event(key) }
        }
    };
    gen.into()
}
