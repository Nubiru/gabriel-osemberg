//! Data models shared between client and server.

pub mod cv_section;
pub mod experience;
pub mod project;
pub mod project_metric;
pub mod project_tag;
pub mod skill;
pub mod tech_tag;

pub use cv_section::CvSection;
pub use experience::Experience;
pub use project::Project;
pub use project_metric::ProjectMetric;
pub use project_tag::ProjectTag;
pub use skill::Skill;
pub use tech_tag::TechTag;
