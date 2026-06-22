#[derive(Debug, Clone, Copy)]
pub struct DebugPrinter {
    verbose: bool,
}

impl DebugPrinter {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.verbose
    }

    pub fn print(&self, message: String) {
        if self.verbose {
            eprintln!("[DEBUG] {message}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DebugPrinter;

    #[test]
    fn reports_when_verbose_is_enabled() {
        let debug = DebugPrinter::new(true);
        assert!(debug.is_enabled());
    }

    #[test]
    fn reports_when_verbose_is_disabled() {
        let debug = DebugPrinter::new(false);
        assert!(!debug.is_enabled());
    }
}
