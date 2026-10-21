#[derive(Clone, PartialEq, Debug)]
pub enum CohortScope {
    SelfScope,
    Family,
    Community,
    Affiliation,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Primitive {
    Scores { dimension: String, score: f32, confidence: f32 },
    DelegatesTo { delegated_scope: Vec<String> },
    Supersedes { target_id: String },
    Withdraws { target_id: String },
    Recants { target_id: String },
}

#[derive(Clone, Debug)]
pub struct Envelope {
    pub id: String,
    pub attesting_key_id: String,
    pub cohort_scope: CohortScope,
    pub primitive: Primitive,
}

pub struct CegEngine {
    pub attestations: Vec<Envelope>,
}

impl CegEngine {
    pub fn new() -> Self {
        Self {
            attestations: Vec::new(),
        }
    }

    pub fn emit(&mut self, env: Envelope) {
        // Handle structural composers
        match &env.primitive {
            Primitive::Withdraws { target_id } | Primitive::Recants { target_id } => {
                self.attestations.retain(|a| a.id != *target_id);
            }
            Primitive::Supersedes { target_id } => {
                self.attestations.retain(|a| a.id != *target_id);
                self.attestations.push(env);
            }
            _ => {
                self.attestations.push(env);
            }
        }
    }

    pub fn get_coherence_score(&self) -> f32 {
        let mut score = 100.0;
        for a in &self.attestations {
            if let Primitive::Scores { dimension, score: s, .. } = &a.primitive {
                if dimension == "infohazard" {
                    score -= 10.0 * s.abs();
                } else if dimension == "good_action" {
                    score += 2.0 * s.abs();
                }
            }
        }
        score.clamp(0.0, 100.0)
    }
}
