pub struct MergeableFeature {
  par: Vec<usize>,
  rank: Vec<usize>,
  meeples: Vec<Vec<i32>>,
  open_sides: Vec<i32>,
  facing_cities: Vec<Vec<usize>>, // for field
  done: Vec<bool>,
}

impl MergeableFeature {
    pub fn new() -> Self {
        MergeableFeature {
          par: vec![],
          rank: vec![],
          meeples: vec![],
          open_sides: vec![],
          facing_cities: vec![],
          done: vec![],
        }
    }
    pub fn new_feature(&mut self, open_side: i32, with_coa: bool) {
      self.par.push(self.par.len());
      let mut r = 1;
      if with_coa {
        r += 1;
      }
      self.rank.push(r);
      self.meeples.push(vec![]);
      self.open_sides.push(open_side);
      self.facing_cities.push(vec![]);
      self.done.push(false);
    }
    pub fn set_cities(&mut self, x: usize, city: usize) {
      self.facing_cities[x].push(city);
    }
    fn root(&mut self, x: usize) -> usize {
        if x != self.par[x] {
            let par = self.par[x];
            let r = self.root(par);
            self.par[x] = r;
        }
        self.par[x]
    }
    pub fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if self.rank[x] > self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        if x == y {
          // when merging already same set, only open_size reduced by 2
          self.open_sides[y] -= 2;
          return
        }
        self.open_sides[y] = self.open_sides[y] + self.open_sides[x] - 2;
        self.par[x] = y;
        self.rank[y] += self.rank[x];
        let mut v = vec![];
        for m in &self.meeples[x] {
          v.push(*m);
        }
        self.meeples[y].append(&mut v);
        // merge facing cities for filed
        let mut v = vec![];
        for c in &self.facing_cities[x] { v.push(*c); }
        for c in &self.facing_cities[y] { v.push(*c); }
        let mut root_v = vec![];
        for e in v {
          root_v.push(self.root(e));
        }
        root_v.sort();
        root_v.dedup();
        self.facing_cities[y] = root_v;
    }
    pub fn is_completed(&mut self, x: usize) -> bool {
      let x = self.root(x);
      self.open_sides[x] == 0
    }
    pub fn place_meeple(&mut self, x: usize, meeple_id: i32) {
      let x = self.root(x);
      assert_eq!(self.meeples[x].len(), 0);
      self.meeples[x].push(meeple_id);
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    pub fn get_meeples(&mut self, x: usize) -> Vec<i32> {
      let x = self.root(x);
      self.meeples[x].clone()
    }
    pub fn get_open_sides(&mut self, x: usize) -> i32 {
      let x = self.root(x);
      self.open_sides[x]
    }
    pub fn get_facing_cities(&mut self, x: usize) -> Vec<usize> {
      let x = self.root(x);
      self.facing_cities[x].clone()
    }
    #[allow(unused)]
    pub fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.rank[x]
    }
    pub fn reduce_open_sides(&mut self, x: usize, count: i32) {
      let x = self.root(x);
      self.open_sides[x] -= count;
    }
    pub fn set_as_done(&mut self, x: usize) {
      let x = self.root(x);
      self.done[x] = true;
    }
    pub fn is_done(&mut self, x: usize) -> bool {
      let x = self.root(x);
      self.done[x]
    }
}
