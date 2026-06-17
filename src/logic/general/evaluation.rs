use crate::logic::general::direction::{Direction, DIRECTIONS};
use std::fmt::{Display, Formatter, Result as FmtResult};
use serde_json;
use tabled::{
    builder::Builder,
    settings::{Alignment, Style, object::Columns},
};

pub struct Evaluation {
    sections: Vec<EvaluationSection>,
    one_line: bool,
}

impl Evaluation {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            one_line: false,
        }
    }

    pub fn one_line(mut self) -> Self {
        self.one_line = true;
        self
    }

    pub fn new_section(&mut self, name: &str) {
        // Start a new section in the evaluation report
        self.sections.push(EvaluationSection {
            name: name.to_string(),
            elimination_priority: [None; 4],
            score_details: std::array::from_fn(|_| Vec::new()),
        });
    }

    pub fn score(&mut self, direction: Direction, score: f64, detail: impl Into<String>) {
        if score == 0.0 {
            return;
        }
        let section = self.sections.last_mut().unwrap();
        let index = direction as usize;
        section.score_details[index].push((score, detail.into()));
    }

    pub fn eliminate(&mut self, direction: Direction, priority: u8) {
        self.sections
            .last_mut()
            .unwrap()
            .elimination_priority[direction as usize] = Some(priority);
    }

    fn directions_after_elimination(&self) -> [bool; 4] {
        let mut directions = [true; 4];
        for section in &self.sections {
            let checkpoint = directions;
            let mut max = None;
            for i in 0..4 {
                if let Some(priority) = section.elimination_priority[i] {
                    if max.is_none() || priority > max.unwrap() {
                        max = Some(priority);
                    }
                    directions[i] = false;
                }
            }
            if directions.iter().all(|&x| !x) {
                for i in 0..4 {
                    if section.elimination_priority[i] == max {
                        directions[i] = checkpoint[i];
                    }
                }
            }

            let available_count = directions.iter().filter(|&&x| x).count();
            if available_count == 1 {
                return directions;
            }
            if available_count == 0 {
                // No direction is valid, fall back to previous checkpoint.
                directions = checkpoint;
            }
        }

        directions
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
    elimination_priority: [Option<u8>; 4],
    score_details: [Vec<(f64, String)>; 4],
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.sections.is_empty() {
            return writeln!(f, "Evaluation (no sections)");
        }

        if self.one_line {
            let totals = self.total_scores();
            let directions = self.directions_after_elimination();
            let picked = self.result();
            let sections: serde_json::Value = self
                .sections
                .iter()
                .map(|s| {
                    let scores: serde_json::Value = DIRECTIONS
                        .iter()
                        .map(|d| {
                            let i = *d as usize;
                            let total: f64 = s.score_details[i].iter().map(|(v, _)| *v).sum();
                            let details: serde_json::Value = s.score_details[i]
                                .iter()
                                .map(|(v, label)| serde_json::json!({label: v}))
                                .collect();
                            (d.to_string(), serde_json::json!({"total": total, "details": details}))
                        })
                        .collect();
                    let elims: serde_json::Value = DIRECTIONS
                        .iter()
                        .filter_map(|d| {
                            s.elimination_priority[*d as usize]
                                .map(|p| (d.to_string(), serde_json::json!(p)))
                        })
                        .collect();
                    (s.name.clone(), serde_json::json!({"scores": scores, "eliminations": elims}))
                })
                .collect();
            let summary: serde_json::Value = DIRECTIONS
                .iter()
                .map(|d| {
                    let i = *d as usize;
                    (d.to_string(), serde_json::json!({
                        "total": totals[i],
                        "available": directions[i],
                        "picked": *d == picked,
                    }))
                })
                .collect();
            let json = serde_json::json!({
                "picked": picked.to_string(),
                "summary": summary,
                "sections": sections,
            });
            return writeln!(f, "{}", json);
        }

        let direction_headers: Vec<String> = DIRECTIONS.iter().map(ToString::to_string).collect();

        let mut elimination_rows: Vec<(String, Vec<String>)> = self
            .sections
            .iter()
            .map(|section| {
                let cells = DIRECTIONS
                    .iter()
                    .map(|direction| match section.elimination_priority[*direction as usize] {
                        Some(priority) => priority.to_string(),
                        None => "-".to_string(),
                    })
                    .collect();
                (section.name.clone(), cells)
            })
            .collect();

        let available_directions = self.directions_after_elimination();
        elimination_rows.push((
            "AVAILABLE".to_string(),
            DIRECTIONS
                .iter()
                .map(|direction| {
                    if available_directions[*direction as usize] {
                        "X".to_string()
                    } else {
                        "".to_string()
                    }
                })
                .collect(),
        ));

        let mut score_rows: Vec<(String, Vec<String>)> = self
            .sections
            .iter()
            .flat_map(|section| {
                let section_has_scores = section
                    .score_details
                    .iter()
                    .any(|details| !details.is_empty());
                if !section_has_scores {
                    return Vec::new();
                }

                let mut rows: Vec<(String, Vec<String>)> = Vec::new();

                let section_total_cells = DIRECTIONS
                    .iter()
                    .map(|direction| {
                        let index = *direction as usize;
                        section.score_details[index]
                            .iter()
                            .map(|(score, _)| *score)
                            .sum::<f64>()
                            .to_string()
                    })
                    .collect();
                rows.push((section.name.clone(), section_total_cells));

                let mut detail_labels: Vec<String> = Vec::new();
                for details in &section.score_details {
                    for (_, detail) in details {
                        if !detail_labels.iter().any(|label| label == detail) {
                            detail_labels.push(detail.clone());
                        }
                    }
                }

                for detail_label in detail_labels {
                    let detail_cells = DIRECTIONS
                        .iter()
                        .map(|direction| {
                            let index = *direction as usize;
                            let value = section.score_details[index]
                                .iter()
                                .filter(|(_, detail)| detail == &detail_label)
                                .map(|(score, _)| *score)
                                .sum::<f64>();
                            value.to_string()
                        })
                        .collect();
                    rows.push((format!("  - {}", detail_label), detail_cells));
                }

                rows
            })
            .collect();

        let totals = self.total_scores();
        score_rows.push((
            "TOTAL".to_string(),
            DIRECTIONS
                .iter()
                .map(|direction| totals[*direction as usize].to_string())
                .collect(),
        ));
        score_rows.push((
            "AVAILABLE".to_string(),
            DIRECTIONS
                .iter()
                .map(|direction| {
                    if available_directions[*direction as usize] {
                        "X".to_string()
                    } else {
                        "".to_string()
                    }
                })
                .collect(),
        ));

        let picked_direction = self.result();
        score_rows.push((
            "PICKED".to_string(),
            DIRECTIONS
                .iter()
                .map(|direction| {
                    if *direction == picked_direction {
                        "X".to_string()
                    } else {
                        "".to_string()
                    }
                })
                .collect(),
        ));

        let mut elimination_builder = Builder::default();
        let mut elimination_header = vec!["Section".to_string()];
        elimination_header.extend(direction_headers.iter().cloned());
        elimination_builder.push_record(elimination_header);
        for (section_name, cells) in &elimination_rows {
            let mut record = vec![section_name.clone()];
            record.extend(cells.iter().cloned());
            elimination_builder.push_record(record);
        }
        let mut elimination_table = elimination_builder.build();
        elimination_table.with(Style::ascii());
        elimination_table.modify(Columns::new(0..=0), Alignment::left());
        elimination_table.modify(Columns::new(1..=direction_headers.len()), Alignment::right());

        let mut score_builder = Builder::default();
        let mut score_header = vec!["Section".to_string()];
        score_header.extend(direction_headers.iter().cloned());
        score_builder.push_record(score_header);
        for (section_name, cells) in &score_rows {
            let mut record = vec![section_name.clone()];
            record.extend(cells.iter().cloned());
            score_builder.push_record(record);
        }
        let mut score_table = score_builder.build();
        score_table.with(Style::ascii());
        score_table.modify(Columns::new(0..=0), Alignment::left());
        score_table.modify(Columns::new(1..=direction_headers.len()), Alignment::right());

        writeln!(f, "Evaluation")?;
        writeln!(f, "Eliminations")?;
        writeln!(f, "{}", elimination_table)?;
        writeln!(f)?;
        writeln!(f, "Scores")?;
        writeln!(f, "{}", score_table)?;

        Ok(())
    }
}