use leptos::prelude::*;

use crate::models::Skill;

/// Displays skills grouped by category with proficiency indicators.
///
/// Categories: language, framework, tool, concept.
/// Proficiency: 1-5 scale, shown as filled dots.
#[component]
pub fn SkillsDisplay(skills: Vec<Skill>) -> impl IntoView {
    let categories = group_by_category(skills);

    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            {categories
                .into_iter()
                .map(|(category, skills)| {
                    let label = category_label(&category);
                    view! { <SkillCategory label=label skills=skills/> }
                })
                .collect_view()}
        </div>
    }
}

/// A single skill category section (e.g., "Languages") with its skills.
#[component]
fn SkillCategory(#[prop(into)] label: String, skills: Vec<Skill>) -> impl IntoView {
    view! {
        <div>
            <h3 class="font-display text-lg font-semibold text-text-primary mb-4">{label}</h3>
            <div class="space-y-3">
                {skills
                    .into_iter()
                    .map(|skill| {
                        let name = skill.name;
                        let proficiency = skill.proficiency;
                        view! { <SkillItem name=name proficiency=proficiency/> }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

/// A single skill with name and proficiency dots.
#[component]
fn SkillItem(#[prop(into)] name: String, proficiency: i64) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between gap-4">
            <span class="text-sm text-text-secondary">{name}</span>
            <div class="flex gap-1" aria-label=format!("Proficiency: {proficiency} of 5")>
                {(1..=5)
                    .map(|i| {
                        let filled = i <= proficiency;
                        view! {
                            <div class=format!(
                                "w-2 h-2 rounded-full {}",
                                if filled { "bg-accent" } else { "bg-surface-overlay" },
                            )></div>
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}

/// Group skills by category, preserving order within each group.
fn group_by_category(skills: Vec<Skill>) -> Vec<(String, Vec<Skill>)> {
    // Maintain a defined order: languages, frameworks, tools, concepts
    let order = ["language", "framework", "tool", "concept"];
    let mut groups: Vec<(String, Vec<Skill>)> = order
        .iter()
        .map(|cat| (cat.to_string(), Vec::new()))
        .collect();

    for skill in skills {
        if let Some(group) = groups.iter_mut().find(|(cat, _)| *cat == skill.category) {
            group.1.push(skill);
        }
    }

    // Remove empty categories
    groups.retain(|(_, skills)| !skills.is_empty());
    groups
}

/// Convert category key to display label.
fn category_label(category: &str) -> String {
    match category {
        "language" => "Languages".to_string(),
        "framework" => "Frameworks".to_string(),
        "tool" => "Tools & Platforms".to_string(),
        "concept" => "Concepts & Methods".to_string(),
        other => {
            let mut chars = other.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_skills() -> Vec<Skill> {
        vec![
            Skill {
                id: 1,
                name: "Rust".into(),
                category: "language".into(),
                proficiency: 2,
                visible: true,
            },
            Skill {
                id: 2,
                name: "C11".into(),
                category: "language".into(),
                proficiency: 5,
                visible: true,
            },
            Skill {
                id: 3,
                name: "Leptos".into(),
                category: "framework".into(),
                proficiency: 2,
                visible: true,
            },
        ]
    }

    #[test]
    fn groups_by_category() {
        let groups = group_by_category(sample_skills());
        assert_eq!(groups.len(), 2); // language + framework
        assert_eq!(groups[0].0, "language");
        assert_eq!(groups[0].1.len(), 2);
        assert_eq!(groups[1].0, "framework");
        assert_eq!(groups[1].1.len(), 1);
    }

    #[test]
    fn empty_categories_removed() {
        let groups = group_by_category(sample_skills());
        // "tool" and "concept" have no skills, should be removed
        assert!(!groups.iter().any(|(cat, _)| cat == "tool"));
        assert!(!groups.iter().any(|(cat, _)| cat == "concept"));
    }

    #[test]
    fn category_labels() {
        assert_eq!(category_label("language"), "Languages");
        assert_eq!(category_label("framework"), "Frameworks");
        assert_eq!(category_label("tool"), "Tools & Platforms");
        assert_eq!(category_label("concept"), "Concepts & Methods");
    }
}
