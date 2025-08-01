use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


/// `InstanceEnum`マクロは、列挙型に`serde::Serialize`と`serde::Deserialize`トレイトを派生させます。
/// これにより、列挙型を`instance-pipe`のメッセージとして利用できます。
#[proc_macro_derive(InstanceEnum)]
pub fn instance_enum_derive(input: TokenStream) -> TokenStream {
    // マクロ入力の解析
    let input = parse_macro_input!(input as DeriveInput);

    // `serde`のderiveマクロを付与するコードを生成
    let expanded = quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #input
    };
    TokenStream::from(expanded)
}

/// `InstanceStruct`マクロは、構造体に`serde::Serialize`と`serde::Deserialize`トレイトを派生させます。
/// これにより、構造体を`instance-pipe`のメッセージとして利用できます。
#[proc_macro_derive(InstanceStruct)]
pub fn instance_struct_derive(input: TokenStream) -> TokenStream {
    // マクロ入力の解析
    let input = parse_macro_input!(input as DeriveInput);

    // `serde`のderiveマクロを付与するコードを生成
    let expanded = quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
        #input
    };
    TokenStream::from(expanded)
}
