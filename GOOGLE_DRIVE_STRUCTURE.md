# MacAllister Polling - Google Drive Folder Structure

## Recommended Organization

```
~/Google Drive/MacAllister Polling/
│
├── 📋 Templates/
│   ├── Polls/
│   │   ├── survey_template.md
│   │   ├── question_design_guidelines.md
│   │   └── polling_questionnaire.md
│   │
│   ├── Reports/
│   │   ├── topline_report_template.md
│   │   ├── verbatim_report_template.md
│   │   ├── targeted_analysis_template.md
│   │   └── executive_summary_template.md
│   │
│   └── Tableau/
│       ├── dashboard_template.twb
│       └── visualization_guide.md
│
├── 📊 Active Projects/
│   ├── [Client Name - Project Name]/
│   │   ├── 01_Proposal/
│   │   ├── 02_Research/
│   │   ├── 03_Polling_Data/
│   │   ├── 04_Analysis/
│   │   └── 05_Deliverables/
│   │
│   └── [Another Client]/
│       └── ...
│
├── 📚 Resources/
│   ├── Best_Practices/
│   │   ├── polling_methodology.md
│   │   ├── question_writing_guide.md
│   │   └── data_analysis_standards.md
│   │
│   ├── Reference_Materials/
│   │   └── industry_research/
│   │
│   └── Training/
│       └── little_helper_guide.md
│
├── 👥 Team/
│   ├── contact_list.md
│   ├── roles_responsibilities.md
│   └── meeting_notes/
│
├── 🎯 Campaigns/
│   └── [Campaign Name]/
│       ├── strategy/
│       ├── content/
│       ├── analytics/
│       └── reports/
│
└── 🗄️ Archive/
    └── [Year]/
        └── [Completed Projects]/
```

---

## Template File Examples

### 1. Poll/Survey Templates

**File:** `Templates/Polls/survey_template.md`

```markdown
# Survey Template

## Project Information
- **Client:** [Client Name]
- **Project:** [Project Name]
- **Date:** [Date]
- **Prepared by:** [Your Name]

## Survey Objectives
[What are we trying to learn?]

## Target Audience
- **Demographics:** [Age, location, etc.]
- **Sample size:** [Number]
- **Methodology:** [Phone, web, panel, etc.]

## Questions

### Section 1: Screening
1. [Screening question 1]
   - [ ] Yes
   - [ ] No

### Section 2: Main Questions
1. [Question 1]
   - Scale: 1-5
   - Very Unlikely → Very Likely

2. [Question 2]
   - Multiple choice:
     - Option A
     - Option B
     - Option C

### Section 3: Demographics
1. Age: ___
2. Location: ___
3. Political affiliation: ___

## Notes
[Any special considerations]
```

---

### 2. Topline Report Template

**File:** `Templates/Reports/topline_report_template.md`

```markdown
# Topline Report

## Executive Summary
**Project:** [Project Name]
**Client:** [Client Name]
**Date:** [Date]
**Sample Size:** [n=XXX]
**Margin of Error:** [±X%]

### Key Findings
1. [Finding 1]
2. [Finding 2]
3. [Finding 3]

---

## Methodology
- **Dates in field:** [Start] to [End]
- **Sample:** [Description]
- **Mode:** [Phone/Web/Mixed]
- **Languages:** [English/Spanish/etc.]

---

## Results

### Question 1: [Question text]
| Response | % |
|----------|---|
| Option A | XX% |
| Option B | XX% |
| Don't know | X% |

**Analysis:** [Brief interpretation]

### Question 2: [Question text]
[Continue for each question]

---

## Cross-Tabulations

### By Age
| Age Group | Favorable | Unfavorable |
|-----------|-----------|-------------|
| 18-34 | XX% | XX% |
| 35-54 | XX% | XX% |
| 55+ | XX% | XX% |

---

## Appendix
- Full questionnaire
- Sample demographics
- Weighting methodology
```

---

### 3. Verbatim Report Template

**File:** `Templates/Reports/verbatim_report_template.md`

```markdown
# Verbatim Report - Open-Ended Responses

**Project:** [Project Name]
**Question:** [The open-ended question asked]
**Total Responses:** [n=XXX]

---

## Themes Identified

### Theme 1: [Theme Name]
**Frequency:** XX% of responses

**Representative Quotes:**
> "Quote 1 from respondent"

> "Quote 2 from respondent"

### Theme 2: [Theme Name]
**Frequency:** XX% of responses

**Representative Quotes:**
> "Quote 1"

> "Quote 2"

---

## Notable Quotes
[Particularly insightful or representative quotes]

---

## Full Verbatim List
[All responses, organized by theme or chronologically]
```

---

### 4. Targeted Analysis Template

**File:** `Templates/Reports/targeted_analysis_template.md`

```markdown
# Targeted Analysis Report

**Segment:** [Demographic/Psychographic Segment]
**Project:** [Project Name]
**Date:** [Date]

---

## Segment Definition
[How is this segment defined? What characteristics?]

---

## Key Insights

### Attitude on Issue X
- **This segment:** XX% favorable
- **General population:** YY% favorable
- **Difference:** [+/- Z%]

**Interpretation:** [Why is this significant?]

---

## Messaging Recommendations
1. **Message A:** [Description]
   - **Resonance:** [Why it works for this segment]

2. **Message B:** [Description]
   - **Resonance:** [Why it works]

---

## Media Consumption
[Where does this segment get information?]
- Platform 1: XX%
- Platform 2: XX%

---

## Recommended Targeting Strategy
[How to reach and persuade this segment]
```

---

## Tableau Template Structure

**File:** `Templates/Tableau/dashboard_template.twb`

This would be an actual Tableau workbook with:
- Pre-configured data connections
- Standard visualizations (bar charts, trend lines, maps)
- Branding (colors, fonts matching MacAllister style)
- Filters and parameters set up

**File:** `Templates/Tableau/visualization_guide.md`

```markdown
# Tableau Visualization Guide

## Standard Charts for Polling Data

### 1. Topline Results
- **Chart type:** Horizontal bar chart
- **Colors:** Blue (favorable), Red (unfavorable), Gray (neutral/DK)
- **Sort:** By largest value

### 2. Trend Over Time
- **Chart type:** Line chart
- **X-axis:** Date
- **Y-axis:** Percentage
- **Include:** Confidence intervals (shaded area)

### 3. Cross-Tabs
- **Chart type:** Grouped bar chart or heatmap
- **Rows:** Demographics (age, gender, etc.)
- **Columns:** Response options

### 4. Geographic Data
- **Chart type:** Map (state or county level)
- **Color scale:** Sequential (blue for favorable, red for unfavorable)

## Branding
- **Primary color:** #[HEX]
- **Secondary color:** #[HEX]
- **Font:** [Font Name]
- **Logo placement:** Top right corner
```

---

## Project Folder Structure Example

```
Active Projects/
└── Sierra Club - Climate Polling 2025/
    ├── 01_Proposal/
    │   ├── proposal_draft.md
    │   ├── budget.xlsx
    │   └── timeline.md
    │
    ├── 02_Research/
    │   ├── background_research.md
    │   ├── competitor_analysis.md
    │   └── literature_review/
    │
    ├── 03_Polling_Data/
    │   ├── questionnaire_final.md
    │   ├── raw_data.csv
    │   ├── weighted_data.csv
    │   └── codebook.md
    │
    ├── 04_Analysis/
    │   ├── topline_report.md
    │   ├── cross_tabs.xlsx
    │   ├── verbatim_analysis.md
    │   └── tableau_dashboard.twb
    │
    └── 05_Deliverables/
        ├── executive_summary.pdf
        ├── full_report.pdf
        ├── presentation.pdf
        └── data_tables.xlsx
```

---

## Little Helper Configuration

Once this structure is set up, Little Helper will:

1. **Pre-load templates** from `Templates/` folder
2. **Quick access:** "Use topline report template" → loads template
3. **Find files:** "Find Sierra Club polling data" → searches Active Projects
4. **AI context:** Can reference best practices from Resources folder

---

## Setup Instructions

### 1. Create the structure:
```bash
# From Google Drive root (on Mac):
cd ~/Google\ Drive/
mkdir -p "MacAllister Polling"/{Templates/{Polls,Reports,Tableau},"Active Projects",Resources/{Best_Practices,Reference_Materials,Training},Team,Campaigns,Archive}
```

### 2. Create template files:
- Copy the markdown templates above into respective folders
- Create Tableau template workbook
- Add your contact list to Team folder

### 3. Configure Little Helper:
Update `settings.json` to point to:
```json
{
  "workspaceRoot": "~/Google Drive/MacAllister Polling",
  "templatesPath": "~/Google Drive/MacAllister Polling/Templates"
}
```

---

## Conventions

### File Naming
- Use underscores: `topline_report.md` ✅ not `topline report.md` ❌
- Include dates when relevant: `polling_data_2025-01-15.csv`
- Version numbers for drafts: `proposal_v1.md`, `proposal_v2.md`

### Markdown for Text Files
- Easy to edit in Little Helper
- Version control friendly
- Can be converted to PDF/Word when needed

### Project Codes (Optional)
- Assign each project a code: `SC-CP-2025` (Sierra Club - Climate Polling - 2025)
- Use in file names for easy filtering

---

*Last updated: 2025-12-12*
