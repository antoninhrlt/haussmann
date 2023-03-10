#[macro_use]
extern crate quote;

use proc_macro::{TokenStream};
//use proc_macro2::Ident;
use syn::{DeriveInput, /*Data, Fields*/};

// /// Creates a macro calling the `::new()` function of the structure 
// /// implementing it, with named parameters.
// /// 
// /// Cannot be implemented on a different statement than structures with named 
// /// field, otherwise the macro has to be directly created.
// #[proc_macro_derive(MacroNew)] 
// pub fn widget_macro(input: TokenStream) -> TokenStream {
//     // Parses the input
//     let ast: DeriveInput = syn::parse(input).unwrap();
    
//     // The identifier of the structure.
//     let actual_ident = ast.ident;
//     // Creates a lowercased identifier.
//     let ident = Ident::new(&actual_ident.to_string().to_lowercase(), actual_ident.span());

//     // The structure's data.
//     let data: Data = ast.data;
    
//     // Only works with structures with named fields.
//     if let Data::Struct(data_struct) = data {
//         if let Fields::Named(fields) = data_struct.fields {
//             // Identifier of each struct field.
//             let mut identifiers: Vec<Ident> = fields.named
//                 .iter()
//                 .map(|field| field.ident.clone().unwrap())
//                 .collect();

//             let first_ident: Ident = identifiers[0].clone();

//             identifiers.remove(0);
            
//             // Creates the macro with every identifier, named.
//             return quote! {
//                 #[doc="Calls the `new()` function with named parameters, in the right order"]
//                 #[macro_export]
//                 macro_rules! #ident {
//                     (#first_ident: $#first_ident:expr #(, #identifiers: $#identifiers:expr)* $(,)*) => {
//                         #actual_ident::new(
//                             $#first_ident,
//                             #($#identifiers,)*
//                         )
//                     };
//                 }
//             }.into();
//         }
//     }

//     panic!("cannot implement the macro derive for this statement");
// }

/// Implements the required traits to make the structure a widget.
#[proc_macro_derive(Widget)]
pub fn widget(input: TokenStream) -> TokenStream { 
    let ast: DeriveInput = syn::parse(input).unwrap();
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

/// Implements the required traits to make the structure a controller widget.
#[proc_macro_derive(Controller)]
pub fn controller(input: TokenStream) -> TokenStream { 
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let struct_ident = ast.ident;

    let gen = quote! {
        impl<T: Widget + 'static> Controller for #struct_ident<T> {
            fn zone(&self) -> &Zone {
                &self.zone
            }

            fn update(&mut self, zone: Zone) {
                self.zone = zone;
            }
        }
        
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
                write!(f, "[instance of a type of controller]")
            }
        }
    };

    gen.into()
}
