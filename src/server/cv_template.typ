// ATS-Friendly CV Template for Gabriel Osemberg
// Generated server-side from database content via typst-as-lib
// Designed for machine parsing: single column, standard headings, no graphics

#import sys: inputs

#set page(
  paper: "a4",
  margin: (top: 2cm, bottom: 2cm, left: 2cm, right: 2cm),
)
#set text(size: 10.5pt)
#set par(justify: true, leading: 0.65em)

// Header
#align(center)[
  #text(size: 18pt, weight: "bold")[#inputs.name]
  #v(4pt)
  #text(size: 10pt)[
    #inputs.location #h(1em) #inputs.phone #h(1em) #inputs.email \
    LinkedIn: #inputs.linkedin #h(1em) GitHub: #inputs.github
  ]
]

#v(8pt)
#line(length: 100%, stroke: 0.5pt)
#v(4pt)

// Summary
== Summary

#inputs.summary

// Experience
== Experience

#for exp in inputs.experiences [
  #v(6pt)
  #grid(
    columns: (1fr, auto),
    align: (left, right),
    text(weight: "bold")[#exp.role],
    text(size: 9pt)[#exp.date_range],
  )
  #text(style: "italic")[#exp.company]
  #v(2pt)
  #exp.description
  #if exp.highlights.len() > 0 [
    #for h in exp.highlights [
      - #h
    ]
  ]
]

// Technical Skills
== Technical Skills

#for cat in inputs.skill_categories [
  *#cat.label*: #cat.skills.join(", ") \
]

// Projects
== Projects

#for proj in inputs.projects [
  #v(4pt)
  *#proj.name* — #proj.tagline \
  #text(size: 9pt, style: "italic")[#proj.tech_stack]
  #if proj.highlights.len() > 0 [
    #for h in proj.highlights [
      - #h
    ]
  ]
]

// Methodology
== AI-Augmented Engineering Methodology

#inputs.methodology

// Military Service
#if inputs.military != none [
  == Military Service

  #inputs.military
]

// Education
== Education

#inputs.education

// Languages
== Languages

#inputs.languages
