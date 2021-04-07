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
        match self.scores.last() {
            Some(&v) => Some(v),
            _ => None,
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None;
        }

        let mut personal_best = self.scores[0];

        for &item in self.scores.iter() {
            if item > personal_best {
                personal_best = item;
            }
        }

        Some(personal_best)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut personal_top_tree = Vec::from(self.scores);
        personal_top_tree.sort_unstable();
        personal_top_tree.reverse();
        personal_top_tree.truncate(3);
        personal_top_tree
    }
}
