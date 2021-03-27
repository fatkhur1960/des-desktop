macro_rules! impl_service {
    ([ $($state:ident),* ]) => {
        use crate::service::error::Error;

        /// Service implementation
        #[derive(Deserialize, PartialEq, Eq, Debug)]
        #[serde(tag = "cmd", rename_all = "camelCase")]
        pub enum ServiceState {
            $($state {
                route: String,
                payload: serde_json::Value,
                callback: String,
                error: String,
            },)*
        }

        pub fn handle_service(webview: &mut tauri::Webview, arg: &str) -> Result<(), String> {
            match serde_json::from_str::<ServiceState>(arg) {
                Err(e) => Err(e.to_string()),
                Ok(state) => {
                    let app_state = crate::service::AppState::new();
                    match state {
                        $(ServiceState::$state {
                            route,
                            payload,
                            callback,
                            error,
                        } => tauri::execute_promise(
                            webview,
                            move || {
                                $state::wire(&app_state, route, payload)
                                    .map_err(|e| ApiResult::error(e.message).into())
                            },
                            callback,
                            error,
                        ),)*
                    }
                    Ok(())
                }
            }
        }

    };
}
