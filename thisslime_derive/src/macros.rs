macro_rules! impl_trait {
    { $trait:ident=> $([ $( $item:tt)+ ])* } => {
        mod methods {
            $( import_item!{ $( $item )* } )*
        }

        pub fn quote(input: &syn::DeriveInput) -> syn::Result<syn::ItemImpl> {
            let items = vec![
                $( quote_item!( $( $item )*, input ) ),*
            ];

            let ident = &input.ident;

            Ok(syn::parse_quote!(
                impl $trait for #ident {
                    #(#items)*
                }
            ))
        }
    };
}

macro_rules! import_item {
    (fn $name:ident()) => {
        mod $name;
        pub use $name::quote as $name;
    };
}

macro_rules! quote_item {
    (fn $name:ident(), $input: ident) => {
        methods::$name($input)?
    };
}
