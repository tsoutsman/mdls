#![deny(
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    meta_variable_misuse,
    missing_debug_implementations,
    non_ascii_idents,
    noop_method_call,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    variant_size_differences,
    clippy::all,
    clippy::pedantic,
    clippy::clone_on_ref_ptr,
    clippy::decimal_literal_representation,
    clippy::empty_structs_with_brackets,
    clippy::rc_buffer,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::str_to_string,
    clippy::undocumented_unsafe_blocks,
    clippy::unreachable,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls
)]

use mdls::{Error, Context, handle};

use lsp_server::Message;
use lsp_types::{notification, request};
use tracing::{info, Level};

fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let _guard = init_logging();

    let mut ctx = Context::init_from_stdio();

    info!("entering main loop");

    while let Some(event) = ctx.next_event() {
        match event {
            Message::Request(request) => {
                macro_rules! dispatch {
                    ($($request:ty => $handler:path),*$(,)?) => {{
                        use ::lsp_types::request::Request;
                        match request.method.as_ref() {
                            $(
                                <$request as Request>::METHOD => match serde_json::from_value::<<$request as Request>::Params>(request.params) {
                                    Ok(params) => match $handler(&mut ctx, params) {
                                        Ok(response) => {
                                            let response: <$request as Request>::Result = response;
                                            lsp_server::Response::new_ok(request.id.clone(), &response)
                                        },
                                        Err(error) => {
                                            let error: Error = error;
                                            error.into()
                                        },
                                    }
                                    Err(_) => {
                                        todo!();
                                    }
                                }
                            )*
                            _ => lsp_server::Response::new_err(
                                request.id,
                                lsp_server::ErrorCode::MethodNotFound as i32,
                                format!("{} method not found", request.method),
                            ),
                        }
                    }}
                }

                ctx.register_request(request.clone());
                let response = dispatch! {
                    request::Formatting => handle::request::format,
                };
                ctx.respond(response);
            }
            Message::Response(response) => {
                ctx.complete_request(response);
            }
            Message::Notification(notification) => {
                macro_rules! dispatch {
                    ($($notification:ty => $handler:path),*$(,)?) => {{
                        use ::lsp_types::notification::Notification;
                        match notification.method.as_ref() {
                            $(
                                <$notification as Notification>::METHOD => {
                                    let params = serde_json::from_value::<<$notification as Notification>::Params>(notification.params)?;
                                    let _: () = $handler(&mut ctx, params)?;
                                }
                            ),*
                            // We don't handle this kind of notification, that's ok.
                            method => {
                                tracing::info!("unrecognised notification: {}", method);
                            }
                        }
                    }}
                }
                dispatch! {
                    notification::DidOpenTextDocument => handle::notification::did_open_text_document,
                    notification::DidChangeTextDocument => handle::notification::did_change_text_document,
                }
            }
        }
    }

    Ok(())
}

fn init_logging() -> tracing_appender::non_blocking::WorkerGuard {
    let appender = tracing_appender::rolling::never("/tmp", "mdls.log");
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(appender);

    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .with_writer(non_blocking_appender)
        .with_max_level(Level::TRACE)
        .with_ansi(false)
        .init();

    guard
}
