#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores { scores: scores.to_vec() }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.clone().last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().cloned().max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores_copy = self.scores.clone();
        scores_copy.sort_by(|a, b| b.cmp(a));
        let slice_length = if scores_copy.len() < 3 {scores_copy.len()} else {3};
        (&scores_copy[0..slice_length]).to_vec()
    }
}
