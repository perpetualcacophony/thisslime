macro_rules! impl_trait {
    { $trait:path=> $([ $( $item:tt)+ ])* } => {
        $( import_item!{ $( $item )* } )*

        pub fn quote(input: &syn::DeriveInput) -> syn::Result<syn::Item> {
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
        pub mod $name;
    };
}

macro_rules! quote_item {
    (fn $name:ident(), $input: ident) => {
        $name::quote($input)?
    };
}
