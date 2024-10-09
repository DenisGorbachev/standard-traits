#[cfg(feature = "proc_macro2_1")]
use proc_macro2_1::{Ident, Span};
#[cfg(feature = "std")]
use std::path::{Path, PathBuf};
#[cfg(any(feature = "syn_2_full", feature = "syn_2_derive"))]
use syn_2::punctuated::Punctuated;
#[cfg(any(feature = "syn_2_full", feature = "syn_2_derive"))]
use syn_2::{AngleBracketedGenericArguments, GenericArgument, GenericParam, Lifetime, LifetimeParam, Token};
#[cfg(feature = "syn_2_full")]
use syn_2::{File, Item};

/// `Provide` is similar to `AsRef`, `Into`, `TryInto`. However, `Provide` allows specifying the return type, so the implementor may choose to return a `T`, `&T`, `Option<T>`, `Result<T, ...>`.
///
/// `Provide` is frequently used with [`Get`] (see its documentation for details).
pub trait Provide<T: ?Sized> {
    type Output;

    fn provide(self) -> Self::Output;
}

#[cfg(feature = "std")]
impl Provide<PathBuf> for &Path {
    type Output = PathBuf;

    fn provide(self) -> PathBuf {
        self.to_path_buf()
    }
}

#[cfg(any(feature = "syn_2_full", feature = "syn_2_derive"))]
impl Provide<LifetimeParam> for syn_2::Lifetime {
    type Output = LifetimeParam;

    fn provide(self) -> LifetimeParam {
        LifetimeParam {
            attrs: vec![],
            lifetime: self,
            colon_token: None,
            bounds: Default::default(),
        }
    }
}

#[cfg(any(feature = "syn_2_full", feature = "syn_2_derive"))]
impl Provide<GenericParam> for Lifetime {
    type Output = GenericParam;

    fn provide(self) -> GenericParam {
        GenericParam::Lifetime(Provide::<LifetimeParam>::provide(self))
    }
}

#[cfg(any(feature = "syn_2_full", feature = "syn_2_derive"))]
impl Provide<AngleBracketedGenericArguments> for Punctuated<GenericArgument, Token![,]> {
    type Output = AngleBracketedGenericArguments;

    fn provide(self) -> AngleBracketedGenericArguments {
        AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Default::default(),
            args: self,
            gt_token: Default::default(),
        }
    }
}

#[cfg(feature = "syn_2_full")]
impl Provide<File> for Item {
    type Output = File;

    fn provide(self) -> File {
        File {
            shebang: None,
            attrs: vec![],
            items: vec![self],
        }
    }
}

#[cfg(feature = "proc_macro2_1")]
impl Provide<Ident> for &str {
    type Output = Ident;

    fn provide(self) -> Ident {
        Ident::new(self, Span::call_site())
    }
}
