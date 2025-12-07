// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

const TAX_PERCENTAGE: f64 = 0.30;
const OWNER_PERCENTAGE: f64 = 0.55;
const PROFIT_PERCENTAGE: f64 = 0.05;
const OPERATING_EXPENSES_PERCENTAGE: f64 = 0.10;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_device_income(move |string|{
        let ui = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        
        let tax = num * TAX_PERCENTAGE;
        let owner = num * OWNER_PERCENTAGE;     
        let profit = num * PROFIT_PERCENTAGE;
        let operating_expenses = num * OPERATING_EXPENSES_PERCENTAGE;

        let result = format!(
            "Steuern: {:.2} EUR\nEigentümeranteil: {:.2} EUR\nGewinn: {:.2} EUR\nBetriebskosten: {:.2} EUR",
            tax, owner, profit, operating_expenses
        );

        ui.set_results(result.into());
    });
    // ui.on_request_increase_value({
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    ui.run()?;

    Ok(())
}
