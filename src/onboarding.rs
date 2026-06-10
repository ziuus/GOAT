use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct OnboardingStatus {
    pub completed: bool,
    pub show_on_first_run: bool,
    pub last_completed_step: String,
    pub skip_provider_setup: bool,
    pub skip_dashboard_setup: bool,
    pub skip_desktop_setup: bool,
}

impl OnboardingStatus {
    pub fn new() -> Self {
        Self {
            completed: false,
            show_on_first_run: true,
            last_completed_step: "".into(),
            skip_provider_setup: false,
            skip_dashboard_setup: false,
            skip_desktop_setup: false,
        }
    }
}
