use sysinfo::System;

pub struct Cpu {
    values: Vec<f32>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
        }
    }

    pub fn update(&mut self, system: &System) -> f32 {
        let usage = system.global_cpu_usage();

        self.values.push(usage);

        if self.values.len() > 5 {
            self.values.remove(0);
        }

        self.values.iter().sum::<f32>() / self.values.len() as f32
    }
}
