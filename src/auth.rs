use std::sync::Arc;

use lighty_launcher::{
    Authenticator,
    auth::{self},
    event::EventBus,
};
use parking_lot::Mutex;
use slint::{ComponentHandle, Model, ModelRc, SharedString, ToSharedString, VecModel, Weak};

use crate::{AccountS, AccountSelectedS, AppState, AppWindow, Logic};

// login para contas online
pub fn on_login_online_account(weak: Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    if let Some(ui) = weak.upgrade() {
        let logic = ui.global::<Logic>();

        logic.on_login_online_account(move |account| {
            let weak = weak.clone();
            let app_state = app_state.clone();

            slint::spawn_local(async_compat::Compat::new(async move {
                if let Some(ui) = weak.upgrade() {
                    let logic = ui.global::<Logic>();

                    let mut auth = auth::MicrosoftAuth::new("00000000402b5328");
                    let event_bus = EventBus::new(1000);
                    let mut rx = event_bus.subscribe();

                    if let Ok(user) = auth.authenticate(Some(&event_bus)).await {
                        logic.set_current_profile(AccountS {
                            mode: "Online".into(),
                            name: user.username.clone().to_shared_string(),
                        });
                        let mut accounts = vec![];

                        accounts.push(AccountS {
                            mode: "Online".to_shared_string(),
                            name: user.username.clone().to_shared_string(),
                        });

                        logic.set_accounts(ModelRc::new(VecModel::from(accounts)));
                        app_state.lock().current_account = Some(user);
                    }

                    match rx.next().await.expect("") {
                        lighty_launcher::event::Event::Auth(auth) => match auth {
                            _ => println!("{:?}", auth),
                        },
                        _ => {}
                    }

                    app_state.lock().current_accounts_mode = crate::ProfileEnum::Online;
                }
            }))
            .ok();
        });
    }
}

pub fn on_create_offline_account(weak: Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    if let Some(ui) = weak.upgrade() {
        let logic = ui.global::<Logic>();

        logic.on_create_offline_account(move |account| {
            let weak = weak.clone();
            let app_state = app_state.clone();

            if app_state
                .lock()
                .accounts
                .iter()
                .any(|u| u.username == account.name.to_string())
            {
                // conta ja existe
                false
            } else {
                slint::spawn_local(async_compat::Compat::new(async move {
                    if let Some(ui) = weak.upgrade() {
                        let logic = ui.global::<Logic>();
                        let event_bus = EventBus::new(1000);
                        let mut rx = event_bus.subscribe();
                        let mut auth = auth::OfflineAuth::new(account.name.to_string());

                        if let Ok(user) = auth.authenticate(Some(&event_bus)).await {
                            // add new account
                            app_state.lock().accounts.push(user);
                            let mut accounts_for_ui = vec![];

                            let accounts = &app_state.lock().accounts;

                            _ = accounts.iter().for_each(|user| {
                                let mode = if user.email.is_none() {
                                    "Offline".to_shared_string()
                                } else {
                                    "Online".to_shared_string()
                                };
                                // set a last account + new account
                                accounts_for_ui.push(AccountS {
                                    mode,
                                    name: user.username.to_shared_string(),
                                });
                            });

                            logic.set_accounts(ModelRc::new(VecModel::from(accounts_for_ui)));
                        }

                        match rx.next().await.expect("") {
                            lighty_launcher::event::Event::Auth(event) => match event {
                                _ => println!("{:?}", event),
                            },
                            lighty_launcher::event::Event::ConsoleOutput(out) => {
                                println!("{:?}", out.line);
                            }
                            _ => {}
                        }
                    }
                }))
                .ok();
            true
            }
        });
    }
}
