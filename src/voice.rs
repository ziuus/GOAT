#![allow(dead_code)]
use crate::config::VoiceConfig;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceInput {
    pub audio_base64: Option<String>,
    pub text_override: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceTranscript {
    pub text: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceOutput {
    pub text: String,
    pub audio_base64: Option<String>,
}

pub struct VoiceSafetyPolicy {
    config: VoiceConfig,
}

impl VoiceSafetyPolicy {
    pub fn new(config: VoiceConfig) -> Self {
        Self { config }
    }

    pub fn can_use_cloud_stt(&self) -> bool {
        self.config.allow_cloud_stt && !self.config.local_only
    }

    pub fn can_use_cloud_tts(&self) -> bool {
        self.config.allow_cloud_tts && !self.config.local_only
    }
}

#[async_trait::async_trait]
pub trait SpeechToTextProvider: Send + Sync {
    fn name(&self) -> &str;
    async fn transcribe(&self, input: &VoiceInput) -> Result<VoiceTranscript>;
    async fn check_health(&self) -> Result<String>;
}

#[async_trait::async_trait]
pub trait TextToSpeechProvider: Send + Sync {
    fn name(&self) -> &str;
    async fn speak(&self, text: &str) -> Result<VoiceOutput>;
    async fn check_health(&self) -> Result<String>;
}

struct ManualStubStt;

#[async_trait::async_trait]
impl SpeechToTextProvider for ManualStubStt {
    fn name(&self) -> &str {
        "manual_stub"
    }

    async fn transcribe(&self, input: &VoiceInput) -> Result<VoiceTranscript> {
        if let Some(txt) = &input.text_override {
            Ok(VoiceTranscript {
                text: txt.clone(),
                confidence: 1.0,
            })
        } else {
            Ok(VoiceTranscript {
                text: "[Simulated audio transcript]".into(),
                confidence: 0.9,
            })
        }
    }

    async fn check_health(&self) -> Result<String> {
        Ok("ManualStub STT is healthy".into())
    }
}

struct ManualStubTts;

#[async_trait::async_trait]
impl TextToSpeechProvider for ManualStubTts {
    fn name(&self) -> &str {
        "manual_stub"
    }

    async fn speak(&self, text: &str) -> Result<VoiceOutput> {
        Ok(VoiceOutput {
            text: text.to_string(),
            audio_base64: None,
        })
    }

    async fn check_health(&self) -> Result<String> {
        Ok("ManualStub TTS is healthy".into())
    }
}

pub struct VoiceManager {
    config: VoiceConfig,
    policy: VoiceSafetyPolicy,
    stt_providers: HashMap<String, Arc<dyn SpeechToTextProvider>>,
    tts_providers: HashMap<String, Arc<dyn TextToSpeechProvider>>,
}

impl VoiceManager {
    pub fn new(config: VoiceConfig) -> Self {
        let policy = VoiceSafetyPolicy::new(config.clone());
        let mut stt_providers: HashMap<String, Arc<dyn SpeechToTextProvider>> = HashMap::new();
        stt_providers.insert("manual_stub".to_string(), Arc::new(ManualStubStt));

        let mut tts_providers: HashMap<String, Arc<dyn TextToSpeechProvider>> = HashMap::new();
        tts_providers.insert("manual_stub".to_string(), Arc::new(ManualStubTts));

        Self {
            config,
            policy,
            stt_providers,
            tts_providers,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    pub async fn check_doctor(&self) -> Result<String> {
        if !self.config.enabled {
            return Ok("Voice is disabled in config.".into());
        }
        let mut status = String::new();
        status.push_str("STT Providers:\n");
        for (name, prov) in &self.stt_providers {
            match prov.check_health().await {
                Ok(h) => status.push_str(&format!("- {}: {}\n", name, h)),
                Err(e) => status.push_str(&format!("- {}: Error - {}\n", name, e)),
            }
        }
        status.push_str("\nTTS Providers:\n");
        for (name, prov) in &self.tts_providers {
            match prov.check_health().await {
                Ok(h) => status.push_str(&format!("- {}: {}\n", name, h)),
                Err(e) => status.push_str(&format!("- {}: Error - {}\n", name, e)),
            }
        }
        Ok(status.trim().to_string())
    }

    pub fn get_providers(&self) -> Vec<String> {
        let mut names = Vec::new();
        for k in self.stt_providers.keys() {
            names.push(format!("STT: {}", k));
        }
        for k in self.tts_providers.keys() {
            names.push(format!("TTS: {}", k));
        }
        names
    }

    pub async fn transcribe(&self, input: &VoiceInput) -> Result<VoiceTranscript> {
        if !self.config.enabled {
            return Err(anyhow!("Voice is disabled"));
        }
        let provider_name = if self.config.stt_provider == "none" {
            "manual_stub"
        } else {
            &self.config.stt_provider
        };

        if let Some(prov) = self.stt_providers.get(provider_name) {
            prov.transcribe(input).await
        } else {
            Err(anyhow!("STT provider {} not found", provider_name))
        }
    }

    pub async fn speak(&self, text: &str) -> Result<VoiceOutput> {
        if !self.config.enabled {
            return Err(anyhow!("Voice is disabled"));
        }
        let provider_name = if self.config.tts_provider == "none" {
            "manual_stub"
        } else {
            &self.config.tts_provider
        };

        if let Some(prov) = self.tts_providers.get(provider_name) {
            prov.speak(text).await
        } else {
            Err(anyhow!("TTS provider {} not found", provider_name))
        }
    }
}
