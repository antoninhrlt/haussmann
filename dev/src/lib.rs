#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Widget)]
pub fn widget(input: TokenStream) -> TokenStream { 
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let struct_ident = ast.ident;

    let gen = quote! {
        impl ToAny for #struct_ident {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }

        impl From<#struct_ident> for Box<dyn Widget> {
            fn from(value: #struct_ident) -> Self {
                Box::new(value)
            }
        }

        impl DebugWidget for #struct_ident {}
    };

    gen.into()
}

#[proc_macro_derive(Controller)]
pub fn controller(input: TokenStream) -> TokenStream { 
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let struct_ident = ast.ident;

    let gen = quote! {
        
        impl<T: Widget + 'static> Widget for #struct_ident<T> {
            fn build(&self) -> Box<dyn Widget> {
                self.widget.build()
            }

            fn colour(&self) -> RGBA {
                self.widget.colour()
            }
        }

        impl<T: Widget + 'static> ToAny for #struct_ident<T> {
            fn as_any(&self) -> &dyn std::any::Any {
                self.widget.as_any()
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
        
        impl<T: Widget + 'static> From<#struct_ident<T>> for Box<dyn Widget> {
            fn from(value: #struct_ident<T>) -> Self {
                Box::new(value)
            }
        }
        
        impl<T: Widget> DebugWidget for #struct_ident<T> {}      
    
        impl<T: Widget> std::fmt::Debug for #struct_ident<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[cannot debug controllers]")
            }
        }
    };

    gen.into()
}
