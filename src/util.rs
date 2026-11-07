//! portablectl model — manage portable services (systemd-portabled).

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub image: String,
    pub running: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub services: Vec<Service>,
}

impl Default for Model {
    fn default() -> Self { Self { services: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn attach(&mut self, name: &str, image: &str) -> bool {
        self.services.push(Service { name: name.into(), image: image.into(), running: false });
        true
    }

    pub fn list(&self) -> &[Service] { &self.services }

    pub fn start(&mut self, name: &str) -> bool {
        if let Some(s) = self.services.iter_mut().find(|s| s.name == name) { s.running = true; true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn attach_start() {
        let mut p = Model::new();
        assert!(p.attach("podman", "/usr/bin/podman"));
        assert!(p.start("podman"));
        assert!(p.list()[0].running);
    }
}
