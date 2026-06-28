use std::sync::Arc;

use lighty_launcher::{
    Authenticator, auth::{self, MicrosoftAuth, OfflineAuth}, event::EventBus,
};
use parking_lot::Mutex;
use slint::{ComponentHandle, Model, ModelRc, SharedString, ToSharedString, VecModel, Weak};

use crate::{Account, AppState, AppWindow, Logic, AccountSelectedS};

pub fn on_create_online_account(weak: Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    if let Some(ui) = weak.upgrade() {
        let logic = ui.global::<Logic>();
        logic.on_login_online_account(move || {
            let weak = weak.clone();
            let app_state = app_state.clone();
            
            slint::spawn_local(async_compat::Compat::new(async move {
                if let Some(ui) = weak.upgrade() {
                    let logic = ui.global::<Logic>();
                
                    let mut auth = auth::MicrosoftAuth::new("");
                    let event_bus= EventBus::new(1000);
                    let mut rx = event_bus.subscribe();

                    if let Ok(user) = auth.authenticate(Some(&event_bus)).await {
                        logic.set_current_profile(Account { mode: "Online".into(), name: user.username.clone().to_shared_string() });
                        let mut accounts = vec![];

                        app_state.lock().accounts.iter().for_each(|a| {
                            let mode = if a.access_token.is_none() {
                                "Offline".to_shared_string()
                            } else {
                                "Online".to_shared_string()
                            };

                            accounts.push(Account { mode, name: user.username.clone().to_shared_string() })
                        });
                        

                        accounts.push(Account { mode: "Online".to_shared_string(), name: user.username.clone().to_shared_string() });

                        logic.invoke_accounts_to(ModelRc::new(VecModel::from(accounts)));
                        //logic.set_accounts(ModelRc::new(VecModel::from(accounts)));

                        app_state.lock().current_account = Some(user);
                    }
                    
                    match rx.next().await.expect("") {
                            lighty_launcher::event::Event::Auth(auth) => {
                                match auth {
                                    _ => println!("{:?}", auth)
                                }
                            }
                            _=> {}
                        }

                    app_state.lock().current_accounts_mode= crate::ProfileEnum::Online;
                }
            }))
            .ok();
        });
    }
}





pub fn on_create_offline_account(weak: Weak<AppWindow>, app_state: Arc<Mutex<AppState>>) {
    if let Some(ui) = weak.upgrade() {
        let logic = ui.global::<Logic>();
        logic.on_create_offline_account(move |name| {
            let weak = weak.clone();
            let app_state = app_state.clone();

            slint::spawn_local(async_compat::Compat::new(async move {
                if let Some(ui) = weak.upgrade() {
                    let logic = ui.global::<Logic>();
                        let event_bus= EventBus::new(1000);
                    let mut rx = event_bus.subscribe();
                    let mut auth = auth::OfflineAuth::new(logic.get_offline_profile_name().to_string());

                    if let Ok(user) = auth.authenticate(Some(&event_bus)).await {
                        let mut accounts = vec![];
                        
                        app_state.lock().accounts.iter().for_each(|a| {
                            let mode = if a.access_token.is_none() {
                                "Offline".to_shared_string()
                            } else {
                                "Online".to_shared_string()
                            };

                            accounts.push(Account { mode, name: user.username.clone().to_shared_string() })
                        });

                        accounts.push(Account { mode: "Offline".to_shared_string(), name: user.username.to_shared_string() });

                        logic.set_accounts(ModelRc::new(VecModel::from(accounts)));
                        app_state.lock().current_account = Some(user);
                    }

                    app_state.lock().current_accounts_mode = crate::ProfileEnum::Offline;

                    match rx.next().await.expect("") {
                        lighty_launcher::event::Event::Auth(event) => {
                            match event {
                                _ => println!("{:?}", event)
                            }
                        }
                        _=> {}
                    }
                }

                
            }))
            .ok();
        });
    }
}


pub fn on_accounts_to_string(weak: Weak<AppWindow>) {
    if let Some(ui) = weak.upgrade() {
        let logic = ui.global::<Logic>();
        logic.on_accounts_to(|accounts| {
            let accounts_to_string = accounts.iter().map(|a|format!("Name: {} Mode: {}", a.name, a.mode).to_shared_string()).collect::<Vec<SharedString>>();
            ModelRc::new(VecModel::from(accounts_to_string))
        });
    }
}