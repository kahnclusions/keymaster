#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod icons;
pub mod ui;

use icons::KeyIcon;
use ui::{Row, Stack};
use sequoia_openpgp::Cert;

pub(crate) struct LocalKeybox {
    pub certs: Vec<Cert>,
}

pub(crate) fn app(cx: Scope<LocalKeybox>) -> Element {
    let window = dioxus_desktop::use_window(cx);
    cx.render(rsx!(
        div {
            header { class: "text-[#565656] bg-[#8BAE68] border-[#e9e9e9] border-b h-12 body-font cursor-pointer", onmousedown: move |_| window.drag(),
                div { class: "ml-16 mr-4 flex flex-wrap p-0 h-full flex-row items-center",
                    a { class: "flex title-font font-medium items-center text-white ml-3",
                        KeyIcon {}
                        span { class: "ml-3 text-xl", "keymaster"}
                    }
                    nav { class: "ml-auto flex flex-wrap items-center text-base justify-center",
                        a { class: "mr-5 hover:text-black", "My Keys"}
                    }
                }
            }

            section { class: "text-gray-900 bg-white body-font p-3",
                Stack {
        body: cx.render(rsx!(cx.props.certs.iter().map(|cert| rsx!(CertEntry { cert: cert }))))
    }
                
            }
        }
    ))
}

#[derive(PartialEq, Props)]
pub(crate) struct CertEntry<'a> {
    cert: &'a Cert,
}

pub(crate) fn CertEntry<'a>(cx: Scope<'a, CertEntry<'a>>) -> Element {
    let attrs: Vec<_> = cx.props.cert.userids().collect();
    let attr = attrs.first().unwrap();

    let name = attr.name().unwrap_or(None).unwrap_or("no_name".to_owned());
    let email = attr
        .email()
        .unwrap_or(None)
        .unwrap_or("no_email".to_owned());

    cx.render(rsx!(Row {
        body: cx.render(rsx!(
            div {
                class: "p-2",
                name
            }
            div {
                class: "p-2",
                email
            }
        ))
    }))
}

