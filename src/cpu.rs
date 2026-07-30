use sysinfo::System;

pub struct Cpu {
    values: Vec<f32>,
    nb_cpu: i16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            nb_cpu: 0,
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
    
    pub fn num_of_cpu(system: &System) -> i16 {
        system.cpus().len() as i16
    }
}
