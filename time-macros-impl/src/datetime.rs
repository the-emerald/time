use crate::{Date, Time};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

pub(crate) struct DateTime {
    date: Date,
    time: Time,
}

impl Parse for DateTime {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            date: Date::parse(input)?,
            time: Time::parse(input)?,
        })
    }
}

impl ToTokens for DateTime {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { date, time } = self;

        tokens.extend(quote! {
            ::time::PrimitiveDateTime::new(#date, #time)
        })
    }
}
