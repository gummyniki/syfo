mod main_ui;

use eframe::egui;
use egui::accesskit::Size;
use egui::emath::Numeric;
use egui::ImeEvent::Disabled;
use egui::RichText;
use sysinfo::Disks;
use sysinfo::System;
use sysinfo::Networks;
use main_ui::*;




impl Default for main_ui::DashboardApp {
    fn default() -> Self {
        let mut sys = System::new_all();
        let mut disks = Disks::new_with_refreshed_list();

        sys.refresh_all();
        sys.refresh_memory();
        let cpu_usage = sys.global_cpu_usage();
        
        let used = sys.used_memory() as f32;
        let total = sys.total_memory() as f32;
        let current_memory = (used / total) * 100.0;
        let networks = Networks::new_with_refreshed_list();
        Self { sys, cpu_usage, current_memory, last_update: std::time::Instant::now(), networks, disks,}
    }
}

impl eframe::App for DashboardApp {

    

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.refresh_every_frame();

        
        egui::CentralPanel::default().show(ctx, |ui| {

            self.start_ui(ui);

    ui.columns(2, |columns| {

        self.cpu_ram_ui(&mut columns[0]);
        self.disks_ui(&mut columns[0]);
        self.networks_ui(&mut columns[1]);


        if self.last_update.elapsed().as_secs() > 2 {
            self.refresh_timed();
        }
        
    });
});
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "Syfo",
        options,
        Box::new(|_cc| Ok(Box::new(DashboardApp::default()))),

    )
}
