use query::Scorer;

#[derive(Clone)]
pub struct MultiTermScorer {
    coords: Vec<f32>,
    idf: Vec<f32>,
    score: f32,
    num_fields: usize,
}

impl MultiTermScorer {
    pub fn new(mut coords: Vec<f32>, idf: Vec<f32>) -> MultiTermScorer {
        coords.insert(0, 0f32);
        MultiTermScorer {
            coords: coords,
            idf: idf,
            score: 0f32,
            num_fields: 0,
        }
    }
 
    pub fn update(&mut self, term_ord: usize, term_freq: u32) {
        self.score += (term_freq as f32) * self.idf[term_ord];
        self.num_fields += 1;
    }

    fn coord(&self,) -> f32 {
        self.coords[self.num_fields]
    }

    pub fn clear(&mut self,) {
        self.score = 0f32;
        self.num_fields = 0;
    }

}


impl Scorer for MultiTermScorer {
    fn score(&self, ) -> f32 {
        self.score * self.coord()
    }
}
