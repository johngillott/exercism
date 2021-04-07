#[derive(Debug)]
pub struct HighScores<'scores> {
    scores: &'scores [u32],
}

// shared lifetime/lifetime chapter

impl<'scores> HighScores<'scores> {
    pub fn new(scores: &'scores [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut vec = self.scores.to_vec();
        vec.sort_unstable_by(|a, b| b.cmp(a));
        vec.truncate(3);
        vec
    }
}
