use crate::{app_config::AppConfig, network_config};
use anyhow::anyhow;
use holochain_client::{AppInfo, CellInfo};
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_holochain::HolochainExt;

pub async fn about_menu<R: tauri::Runtime>(h: &AppHandle<R>) {
    let app_version = h.package_info().version.to_string();

    let app_info_message = match get_app_info(h.clone()).await {
        Ok((_all_apps, expected_app_info)) => {
            // // create a pretty list of installed apps
            // let list_of_installed_apps = all_apps
            //     .iter()
            //     .map(|app| format!("{:?}", app))
            //     .collect::<Vec<String>>()
            //     .join("\n");
            // let list_of_installed_apps = list_of_installed_apps
            //     .lines()
            //     .map(|line| format!("\t\t➤ {}\n", line))
            //     .collect::<Vec<String>>()
            //     .join("\n");
            // create a pretty visual string of the expected_app_info and check if it is installed
            struct Details {
                app_name: String,
                dna_hash: String,
                network_seed: String,
                agent_pub_key: String,
                status: String,
            }
            let message = match expected_app_info {
                Some(app_info) => {
                    let existing_cells = app_info.cell_info.get("alliance");

                    let existing_cell = existing_cells
                        .cloned()
                        .unwrap_or_default()
                        .iter()
                        .find_map(|c| match c {
                            CellInfo::Provisioned(c) => Some(c.clone()),
                            _ => None,
                        });

                    let details = Details {
                        app_name: app_info.installed_app_id.as_str().to_string(),
                        dna_hash: if let Some(cell) = existing_cell.clone() {
                            cell.cell_id.dna_hash().to_string()
                        } else {
                            "not found".to_string()
                        },
                        network_seed: if let Some(cell) = existing_cell.clone() {
                            cell.dna_modifiers.network_seed.to_string()
                        } else {
                            "not found".to_string()
                        },
                        agent_pub_key: format!("{:?}", app_info.agent_pub_key),
                        status: format!("{:?}", app_info.status),
                    };
                    format!(
                        "\t\t➤ Name:  {:?}\n\t\t➤ dna hash: {:?}\n\t\t➤ network seed: {:?}\n\t\t➤ agent pub key: {:?}\n\t\t➤ status: {:?}",
                        details.app_name,
                        details.dna_hash,
                        details.network_seed,
                        details.agent_pub_key,
                        details.status
                    )
                }
                None => "App that is expected to be installed is not installed".to_string(),
            };
            message
            // let app_name = expected_app_info
            //     .iter()
            //     .map(|app| app.installed_app_id.as_str())
            //     .collect::<Vec<&str>>()
            //     .join("\n");
            // let app_name = app_name
            //     .lines()
            //     .map(|line| format!("\t\t➤ {}\n", line))
            //     .collect::<Vec<String>>()
            //     .join("\n");
            // let app_name = if app_name.is_empty() {
            //     "\tUnable to find app".to_string()
            // } else {
            //     app_name
            // };
            // format!(
            //     "\tExpected App Info:\n{}\n",
            //     list_of_installed_apps, expected_app_info
            // )
        }
        Err(e) => format!("Error getting app info: {:?}", e),
    };

    let network_config = network_config();

    let product_name = AppConfig::new(h).product_name;
    let about_message = format!(
        "{} Version: v{}\n\n---\n\nApp Info:\n{}\n\n---\n\n{:#?}",
        product_name, app_version, app_info_message, network_config,
    );

    h.dialog()
        .message(about_message)
        .title(format!("About {}", product_name))
        .show(|_| {})
}

async fn get_app_info<R: tauri::Runtime>(
    handle: AppHandle<R>,
) -> anyhow::Result<(Vec<AppInfo>, Option<AppInfo>)> {
    match handle.holochain() {
        Ok(holochain_client) => {
            match holochain_client.admin_websocket().await {
                Ok(admin_ws) => {
                    // Check if the expected app is installed
                    let installed_apps = admin_ws
                        .list_apps(None)
                        .await
                        .map_err(|err| tauri_plugin_holochain::Error::ConductorApiError(err))?;
                    let handle = handle.clone();
                    let expected_app_info = installed_apps.clone().into_iter().find(|app| {
                        app.installed_app_id
                            .as_str()
                            .eq(&AppConfig::new(&handle).app_id)
                    });

                    // Check if an old version is still installed
                    return Ok((installed_apps.clone(), expected_app_info));
                }
                Err(e) => return Err(anyhow!("Error getting Holochain client: {:?}", e)),
            }
        }
        Err(e) => return Err(anyhow!("Error getting Holochain client: {:?}", e)),
    }
}
