//extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Парсим входные строки
    let inputs = parse_macro_input!(input with syn::punctuated::Punctuated::<LitStr, syn::Token![,]>::parse_terminated);

    // Вектор для хранения вызовов функций
    let mut function_calls = Vec::new();

    // Проходим по каждому имени функции
    for lit in inputs.iter() {
        let func_name = lit.value();
        let length = func_name.len();

        // Проверяем, если длина имени функции чётная
        if length % 2 == 0 {
            // Создаём вызов функции
            let ident = syn::Ident::new(&func_name, lit.span());
            function_calls.push(quote! {
                #ident()
            });
        }
    }

    // Если нет функций с чётным количеством символов
    if function_calls.is_empty() {
        function_calls.push(quote! {
            ()
        });
    }

    // Создаём код для возвращения кортежа
    let output = quote! {
        (#(#function_calls),*)
    };

    output.into()
}





