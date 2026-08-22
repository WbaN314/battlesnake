use crate::logic::general::direction::{Direction, DIRECTIONS};
use std::env;
use std::fmt::{Display, Formatter, Result as FmtResult};
use tabled::{
    builder::Builder,
    settings::{Alignment, Style, object::Columns},
};

pub struct Evaluation {
    sections: Vec<EvaluationSection>,
    one_line: bool,
    enabled: bool,
}

impl Evaluation {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            one_line: false,
            enabled: false,
        }
    }

    pub fn from_env() -> Self {
        match env::var("LOG_EVAL").as_deref() {
            Ok("full") => Self { sections: Vec::new(), one_line: false, enabled: true },
            Ok("oneline") => Self { sections: Vec::new(), one_line: true, enabled: true },
            _ => Self::new(),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn one_line(mut self) -> Self {
        self.one_line = true;
        self
    }

    pub fn is_eliminated(&self, direction: Direction) -> bool {
        let index = direction as usize;
        self.directions_after_elimination()[index] == false
    }

    pub fn new_section(&mut self, name: &str) {
        // Start a new section in the evaluation report
        self.sections.push(EvaluationSection {
            name: name.to_string(),
            elimination_score: std::array::from_fn(|_| None),
            score_details: std::array::from_fn(|_| Vec::new()),
        });
    }

    pub fn score(&mut self, direction: Direction, score: f64, detail: impl Into<String>) {
        let section = self.sections.last_mut().unwrap();
        let index = direction as usize;
        section.score_details[index].push((score, detail.into()));
    }

    pub fn eliminate(&mut self, direction: Direction, score: u8, detail: impl Into<String>) {
        self.sections
            .last_mut()
            .unwrap()
            .elimination_score[direction as usize] = Some((score, detail.into()));
    }

    fn directions_after_elimination(&self) -> [bool; 4] {
        self.directions_after_each_section().into_iter().last().unwrap_or([true; 4])
    }

    fn directions_after_each_section(&self) -> Vec<[bool; 4]> {
        let mut directions = [true; 4];
        let mut result = Vec::new();
        for section in &self.sections {
            let checkpoint = directions;
            let mut max = None;
            for i in 0..4 {
                if let Some((priority, _)) = section.elimination_score[i] {
                    if max.is_none() || priority > max.unwrap() {
                        max = Some(priority);
                    }
                    directions[i] = false;
                }
            }
            if directions.iter().all(|&x| !x) {
                for i in 0..4 {
                    if section.elimination_score[i].as_ref().map(|(p, _)| *p) == max {
                        directions[i] = checkpoint[i];
                    }
                }
            }

            let available_count = directions.iter().filter(|&&x| x).count();
            if available_count == 0 {
                // No direction is valid, fall back to previous checkpoint.
                directions = checkpoint;
            }
            result.push(directions);
            if available_count == 1 {
                while result.len() < self.sections.len() {
                    result.push(directions);
                }
                return result;
            }
        }
        result
    }

    pub fn result(&self) -> Direction {
        let scores = self.total_scores();
        let directions = self.directions_after_elimination();

        if directions.iter().filter(|&&x| x).count() == 1 {
            let value = directions.iter().enumerate().find(|&(_, &x)| x).unwrap().0;
            return Direction::try_from(value).unwrap();
        }

        // Return direction with highest score that is not eliminated
        let mut best_direction = None;
        for i in 0..4 {
            if directions[i] {
                if best_direction.is_none() || scores[i] > scores[best_direction.unwrap() as usize] {
                    best_direction = Some(i);
                }
            }
        }
        if let Some(best) = best_direction {
            return Direction::try_from(best).unwrap();
        }

        Direction::Up
    }

    fn total_scores(&self) -> [f64; 4] {
        let mut totals = [0.0; 4];
        for section in &self.sections {
            for (i, details) in section.score_details.iter().enumerate() {
                totals[i] += details.iter().map(|(score, _)| *score).sum::<f64>();
            }
        }
        totals
    }

}

struct EvaluationSection {
    name: String,
    elimination_score: [Option<(u8, String)>; 4],
    score_details: [Vec<(f64, String)>; 4],
}

fn fmt_score(v: f64) -> String {
    if v == 0.0 { "0".to_string() } else { v.to_string() }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.sections.is_empty() {
            return writeln!(f, "Evaluation (no sections)");
        }

        if self.one_line {
            let picked = self.result();
            // Collect all unique (section, reason) labels in order of first appearance
            let mut reason_labels: Vec<(usize, String)> = Vec::new();
            for (si, s) in self.sections.iter().enumerate() {
                for details in &s.score_details {
                    for (_, label) in details {
                        if !reason_labels.iter().any(|(ri, rl)| *ri == si && rl == label) {
                            reason_labels.push((si, label.clone()));
                        }
                    }
                }
            }
            let parts: Vec<String> = reason_labels
                .iter()
                .map(|(si, label)| {
                    let s = &self.sections[*si];
                    let scores: Vec<String> = DIRECTIONS
                        .iter()
                        .map(|d| {
                            let total: f64 = s.score_details[*d as usize]
                                .iter()
                                .filter(|(_, l)| l == label)
                                .map(|(v, _)| *v)
                                .sum();
                            fmt_score(total)
                        })
                        .collect();
                    format!("{} {}", label, scores.join(" "))
                })
                .collect();
            let totals = self.total_scores();
            let total_part = format!(
                "Total {}",
                DIRECTIONS.iter().map(|d| fmt_score(totals[*d as usize])).collect::<Vec<_>>().join(" ")
            );
            let directions = self.directions_after_elimination();
            let eliminated: Vec<String> = DIRECTIONS
                .iter()
                .filter(|d| !directions[**d as usize])
                .map(|d| {
                    let reason = self.sections.iter()
                        .find_map(|s| s.elimination_score[*d as usize].as_ref().map(|(_, detail)| detail.clone()))
                        .unwrap_or_else(|| "?".to_string());
                    format!("{}({})", d, reason)
                })
                .collect();
            let elim_part = if eliminated.is_empty() {
                "Eliminated none".to_string()
            } else {
                format!("Eliminated {}", eliminated.join(" "))
            };
            return write!(f, "{} | {} | {} | Picked {}\n", parts.join(" | "), total_part, elim_part, picked);
        }

        let direction_headers: Vec<String> = DIRECTIONS.iter().map(ToString::to_string).collect();

        let available_directions = self.directions_after_elimination();
        let directions_per_section = self.directions_after_each_section();

        let mut rows: Vec<(String, Vec<String>)> = self
            .sections
            .iter()
            .enumerate()
            .flat_map(|(section_idx, section)| {
                let section_directions = directions_per_section.get(section_idx).copied().unwrap_or([true; 4]);
                let prior_directions = if section_idx == 0 {
                    [true; 4]
                } else {
                    directions_per_section.get(section_idx - 1).copied().unwrap_or([true; 4])
                };
                let section_has_scores = section.score_details.iter().any(|d| !d.is_empty());
                let section_has_eliminations = section.elimination_score.iter().any(|p| p.is_some());
                if !section_has_scores && !section_has_eliminations {
                    return Vec::new();
                }

                let mut rows: Vec<(String, Vec<String>)> = Vec::new();

                let section_total_cells = DIRECTIONS
                    .iter()
                    .map(|direction| {
                        let index = *direction as usize;
                        let score = fmt_score(section.score_details[index].iter().map(|(s, _)| *s).sum::<f64>());
                        if prior_directions[index] && !section_directions[index] {
                            "X".to_string()
                        } else if score == "0" {
                            "".to_string()
                        } else {
                            score
                        }
                    })
                    .collect();
                rows.push((section.name.clone(), section_total_cells));

                let mut elim_labels: Vec<String> = Vec::new();
                for opt in &section.elimination_score {
                    if let Some((_, label)) = opt {
                        if !elim_labels.contains(label) {
                            elim_labels.push(label.clone());
                        }
                    }
                }
                for label in elim_labels {
                    let cells = DIRECTIONS
                        .iter()
                        .map(|direction| {
                            match &section.elimination_score[*direction as usize] {
                                Some((p, l)) if l == &label => p.to_string(),
                                _ => "".to_string(),
                            }
                        })
                        .collect();
                    rows.push((format!("  ! {}", label), cells));
                }

                let mut detail_labels: Vec<String> = Vec::new();
                for details in &section.score_details {
                    for (_, label) in details {
                        if !detail_labels.contains(label) {
                            detail_labels.push(label.clone());
                        }
                    }
                }
                for label in detail_labels {
                    let cells = DIRECTIONS
                        .iter()
                        .map(|direction| {
                            let index = *direction as usize;
                            let entries: Vec<f64> = section.score_details[index]
                                .iter()
                                .filter(|(_, l)| l == &label)
                                .map(|(s, _)| *s)
                                .collect();
                            if entries.is_empty() {
                                "".to_string()
                            } else {
                                fmt_score(entries.iter().sum())
                            }
                        })
                        .collect();
                    rows.push((format!("  - {}", label), cells));
                }

                rows
            })
            .collect();

        let totals = self.total_scores();
        rows.push((
            "TOTAL".to_string(),
            DIRECTIONS.iter().map(|d| fmt_score(totals[*d as usize])).collect(),
        ));
        rows.push((
            "AVAILABLE".to_string(),
            DIRECTIONS
                .iter()
                .map(|d| if available_directions[*d as usize] { "X".to_string() } else { "".to_string() })
                .collect(),
        ));
        let picked_direction = self.result();
        rows.push((
            "PICKED".to_string(),
            DIRECTIONS
                .iter()
                .map(|d| if *d == picked_direction { "X".to_string() } else { "".to_string() })
                .collect(),
        ));

        let mut builder = Builder::default();
        let mut header = vec!["Section".to_string()];
        header.extend(direction_headers.iter().cloned());
        builder.push_record(header);
        for (label, cells) in &rows {
            let mut record = vec![label.clone()];
            record.extend(cells.iter().cloned());
            builder.push_record(record);
        }
        let mut table = builder.build();
        table.with(Style::ascii());
        table.modify(Columns::new(0..=0), Alignment::left());
        table.modify(Columns::new(1..=direction_headers.len()), Alignment::right());

        writeln!(f, "Evaluation")?;
        writeln!(f, "{}", table)?;

        Ok(())
    }
}