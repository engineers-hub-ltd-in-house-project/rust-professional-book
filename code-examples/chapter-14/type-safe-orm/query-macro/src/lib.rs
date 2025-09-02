use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Expr, LitStr, Token, Type};

struct QueryAsInput {
    output_type: Type,
    _comma1: Token![,],
    sql: LitStr,
    _comma2: Option<Token![,]>, // オプショナルなカンマ
    params: Punctuated<Expr, Token![,]>,
}

impl syn::parse::Parse for QueryAsInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            output_type: input.parse()?,
            _comma1: input.parse()?,
            sql: input.parse()?,
            _comma2: input.parse()?,
            params: Punctuated::parse_terminated(input)?,
        })
    }
}

#[proc_macro]
pub fn query_as_validated(input: TokenStream) -> TokenStream {
    let QueryAsInput {
        output_type,
        sql,
        params,
        .. 
    } = parse_macro_input!(input as QueryAsInput);

    // 本来の実装では、ここで魔法が起きます：
    // 1. `.sqlx`ディレクトリから事前に保存されたスキーマ情報を読み込む。
    // 2. `sql`文字列をパースする。
    // 3. SQLをスキーマと照合して検証する。
    // 4. カラムと型が`output_type`構造体と一致するか確認する。
    // 5. バインドパラメータ(`?`)が`params`の型と一致するか確認する。
    // 6. 検証に失敗した場合、`syn::Error::new(...).to_compile_error()`でコンパイルエラーを返す。
    //
    // このプロジェクトでは、マクロの構造を示すため、検証をパスしたと仮定し、
    // `sqlx::query_as!`マクロが生成するであろうコードを直接生成します。

    let expanded = quote! {
        sqlx::query_as!(#output_type, #sql, #params)
    };

    expanded.into()
}
