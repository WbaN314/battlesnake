use crate::logic::depth_first::game::d_direction::DDirection;

pub struct DScores {
    scores: [Vec<f64>; 4],
}

impl DScores {
    pub fn new() -> Self {
        DScores {
            scores: [Vec::new(), Vec::new(), Vec::new(), Vec::new()],
        }
    }

    pub fn push(&mut self, direction: DDirection, score: f64) {
        self.scores[direction as usize].push(score);
    }

    pub fn evaluate(&self) -> DDirection {
        let mut viable = [true; 4];
        let max_length = self.scores.iter().map(|x| x.len()).max().unwrap();
        for value_index in 0..max_length {
            let mut to_beat = f64::MIN;
            for direction_index in 0..4 {
                if viable[direction_index] {
                    if let Some(value) = self.scores[direction_index].get(value_index) {
                        to_beat = to_beat.max(*value);
                    }
                }
            }
            for direction_index in 0..4 {
                if viable[direction_index] {
                    if let Some(value) = self.scores[direction_index].get(value_index) {
                        if *value < to_beat {
                            viable[direction_index] = false;
                        }
                    } else {
                        viable[direction_index] = false;
                    }
                }
            }
        }
        for i in 0..4 {
            if viable[i] {
                return i.try_into().unwrap();
            }
        }
        panic!("No viable direction found");
    }
}

#[cfg(test)]
mod tests {
    use super::DScores;
    use crate::logic::depth_first::game::d_direction::DDirection;

    #[test]
    fn test_scores() {
        let mut scores = DScores::new();
        scores.push(DDirection::Up, 1.);
        scores.push(DDirection::Down, 1.);
        scores.push(DDirection::Up, 2.);
        scores.push(DDirection::Down, -2.);
        assert_eq!(scores.evaluate(), DDirection::Up);

        let mut scores = DScores::new();
        scores.push(DDirection::Up, 1.);
        scores.push(DDirection::Down, 1.);
        scores.push(DDirection::Up, -2.);
        scores.push(DDirection::Down, -2.);
        scores.push(DDirection::Left, 2.);
        scores.push(DDirection::Left, 2.);
        assert_eq!(scores.evaluate(), DDirection::Left);

        let mut scores = DScores::new();
        scores.push(DDirection::Right, 3.);
        scores.push(DDirection::Left, -1.);
        scores.push(DDirection::Down, 0.);
        assert_eq!(scores.evaluate(), DDirection::Right);

        let mut scores = DScores::new();
        scores.push(DDirection::Up, 1.);
        scores.push(DDirection::Down, -1.);
        scores.push(DDirection::Left, -1.);
        scores.push(DDirection::Right, -1.);
        assert_eq!(scores.evaluate(), DDirection::Up);
    }
}
