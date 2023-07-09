use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Pos(i32, i32);

impl Pos {
    fn successors(&self) -> Vec<Pos> {
        let &Pos(x, y) = self;
        vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
             Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
    }
}

pub fn find() -> Option<Vec<Pos>>  {
    static GOAL: Pos = Pos(4, 6);
    bfs(&Pos(1, 1), |p| p.successors(), |p| *p == GOAL)
}