use egui::RichText;
use sysinfo::{Disks, Networks, System};


pub struct DashboardApp {
    pub sys: System,
    pub cpu_usage: f32,
    pub current_memory: f32,
    pub last_update: std::time::Instant,
    pub networks: Networks,
    pub disks: Disks,

}

impl DashboardApp {
    pub fn start_ui(&self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("Welcome to Syfo!").size(40.0).strong());

        ui.heading(RichText::new(("Up time: ").to_string() + &(System::uptime() / 60).to_string() + " minutes").size(15.0).strong());
        ui.heading(RichText::new(("OS version: ").to_string() + System::long_os_version().unwrap().as_str()));

        ui.add_space(40.0);

        ui.heading(RichText::new("Usages:").size(20.0).strong());
        ui.add_space(20.0);

    }

    pub fn refresh_timed(&mut self) {
        self.networks = Networks::new_with_refreshed_list();
        self.last_update = std::time::Instant::now();
        self.disks = Disks::new_with_refreshed_list();

        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        System::refresh_all(&mut self.sys);
    }

    pub fn refresh_every_frame(&mut self) {
        self.cpu_usage = self.sys.global_cpu_usage();

        let used = self.sys.used_memory() as f32;
        let total = self.sys.total_memory() as f32;
        self.current_memory = (used / total) * 100.0;
    }

    pub fn cpu_ram_ui(&self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new(format!("CPU Usage: {:.2}%", self.cpu_usage))
                .size(15.0)
                .color(egui::Color32::LIGHT_GREEN)
                .strong(),
        );


        ui.add_space(20.0);

        ui.label(
            RichText::new(format!("RAM Usage: {:.2}%", self.current_memory))
                .size(15.0)
                .color(egui::Color32::LIGHT_BLUE)
                .strong(),

        );

        ui.add_space(40.0);
    }

    pub fn disks_ui(&self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("Disks:").size(20.0).strong());
        ui.add_space(20.0);


        for disk in self.disks.list() {
            ui.label(
                RichText::new(format!(
                    "Disk Name: {:?} Kind: {:?} Size: {:.2} GB",
                    disk.name(),
                    disk.kind(),
                    disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0
                ))
                    .size(15.0)
                    .color(egui::Color32::ORANGE)
                    .strong(),
            );
            ui.add_space(10.0);
        }
    }

    pub fn networks_ui(&self, ui: &mut egui::Ui) {
        ui.heading(RichText::new("Networks:").size(20.0).strong());
        ui.add_space(20.0);

        for (interface_name, network) in &self.networks {
            ui.label(
                RichText::new(format!(
                    "{} — Received: {:.2} MB, Transmitted: {:.2} MB",
                    interface_name,
                    network.received() as f32 / 1024.0 / 1024.0,
                    network.transmitted() as f32 / 1024.0 / 1024.0
                ))
                    .size(15.0)
                    .strong()
                    .color(egui::Color32::GREEN)
            );
            ui.add_space(10.0);
        }

    }
}

