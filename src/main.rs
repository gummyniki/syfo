use eframe::egui;
use egui::accesskit::Size;
use egui::emath::Numeric;
use egui::RichText;
use sysinfo::Disks;
use sysinfo::System;
use sysinfo::Networks;


struct DashboardApp {
    sys: System,
    cpu_usage: f32,
    current_memory: f32,
    last_update: std::time::Instant,
    networks: Networks,

}





impl Default for DashboardApp {
    fn default() -> Self {
        let mut sys = System::new_all();

        sys.refresh_all();
        sys.refresh_memory();
        let cpu_usage = sys.global_cpu_usage();
        
        let used = sys.used_memory() as f32;
        let total = sys.total_memory() as f32;
        let current_memory = (used / total) * 100.0;
        let networks = Networks::new_with_refreshed_list();
        Self { sys, cpu_usage, current_memory, last_update: std::time::Instant::now(), networks,}  
    }
}

impl eframe::App for DashboardApp {

    

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        


        self.cpu_usage = self.sys.global_cpu_usage();

        let used = self.sys.used_memory() as f32;
        let total = self.sys.total_memory() as f32;
        let disks = Disks::new_with_refreshed_list();
        self.current_memory = (used / total) * 100.0;

        
        egui::CentralPanel::default().show(ctx, |ui| {
    ui.heading(RichText::new("Welcome to Syfo!").size(40.0).strong());
    
    ui.heading(RichText::new(("Up time: ").to_string() + &(System::uptime() / 60).to_string() + " minutes").size(15.0).strong());
    ui.heading(RichText::new(("OS version: ").to_string() + System::long_os_version().unwrap().as_str()));

    ui.add_space(40.0);

    ui.heading(RichText::new("Usages:").size(20.0).strong());
    ui.add_space(20.0);

    

    ui.columns(2, |columns| {


        

        // First column: CPU & RAM
        columns[0].label(
            RichText::new(format!("CPU Usage: {:.2}%", self.cpu_usage))
                .size(15.0)
                .color(egui::Color32::LIGHT_GREEN)
                .strong(),
        );




        columns[0].add_space(20.0);

        columns[0].label(
            RichText::new(format!("RAM Usage: {:.2}%", self.current_memory))
                .size(15.0)
                .color(egui::Color32::LIGHT_BLUE)
                .strong(),
                
        );

        columns[0].add_space(40.0);
        
        columns[0].heading(RichText::new("Disks:").size(20.0).strong());
        columns[0].add_space(20.0);


        for disk in disks.list() {
            columns[0].label(
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
            columns[0].add_space(10.0);
        }


        if self.last_update.elapsed().as_secs() > 2 {
            self.networks = Networks::new_with_refreshed_list();
            self.last_update = std::time::Instant::now();

            self.sys.refresh_cpu_all();
            self.sys.refresh_memory();
            System::refresh_all(&mut self.sys);

        }

        

        columns[1].heading(RichText::new("Networks:").size(20.0).strong());
        columns[1].add_space(20.0);

        for (interface_name, network) in &self.networks {
    columns[1].label(
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
    columns[1].add_space(10.0);
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
