//! Data models shared between client and server.

pub mod cv_section;
pub mod experience;
pub mod project;
pub mod skill;

pub use cv_section::CvSection;
pub use experience::Experience;
pub use project::Project;
pub use skill::Skill;
