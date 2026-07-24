use std::cmp::Reverse;

#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
    top_three: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let mut sorted_scores = scores.to_vec();
        sorted_scores.sort_unstable_by_key(|&num| {Reverse(num)});
        sorted_scores.truncate(3);
        Self {
            scores: scores.to_vec(),
            top_three: sorted_scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.top_three.first().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.top_three.clone()
    }
}
