pub struct ActiveSkillSession {
    pub execution: crate::skill_runner::SkillExecution,
    pub skill: crate::skills::Skill,
    pub steps: Vec<crate::skill_runner::SkillStep>,
}
