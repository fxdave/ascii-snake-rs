use crate::common::{Pos, Size};
use crate::game::{Draw, DrawInstruction, IsFreePos, MapTrait, SizeGet};
use std::mem;

#[derive(Clone)]
enum MapElement {
    Empty,
    Block,
}

pub struct Map {
    content: Vec<Vec<MapElement>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut content = vec![vec![MapElement::Empty; width]; height];

        for line in &mut content {
            let _ = mem::replace(line.first_mut().unwrap(), MapElement::Block);
            let _ = mem::replace(line.last_mut().unwrap(), MapElement::Block);
        }
        let _ = mem::replace(content.first_mut().unwrap(), vec![MapElement::Block; width]);
        let _ = mem::replace(content.last_mut().unwrap(), vec![MapElement::Block; width]);

        Map { content }
    }
}

impl IsFreePos for Map {
    fn is_free_pos(&self, pos: Pos) -> bool {
        match self.content.get(pos.y).and_then(|row| row.get(pos.x)) {
            Some(MapElement::Block) => false,
            Some(MapElement::Empty) => true,
            None => panic!("Cell not found on {:?}", pos),
        }
    }
}

impl Draw for Map {
    fn draw(&self) -> Vec<DrawInstruction> {
        self.content
            .iter()
            .enumerate()
            .map(|(y, row)| {
                DrawInstruction(
                    Pos { x: 0, y },
                    row.iter()
                        .map(|el| match el {
                            MapElement::Empty => " ",
                            MapElement::Block => "#",
                        })
                        .collect(),
                )
            })
            .collect()
    }
}

impl SizeGet for Map {
    fn get_size(&self) -> Size {
        Size {
            x: self.content.get(0).expect("Empty map").len(),
            y: self.content.len(),
        }
    }
}

impl MapTrait for Map {}

#[cfg(test)]
mod test {
    use super::Draw;
    use super::DrawInstruction;
    use super::IsFreePos;
    use super::Map;
    use super::Pos;

    #[test]
    fn it_can_make_a_new_map() {
        let map = Map::new(20, 20);
        assert_eq!(map.content.len(), 20);
        assert_eq!(map.content.get(0).unwrap().len(), 20);
    }

    #[test]
    fn it_can_check_collision() {
        let map = Map::new(20, 20);
        assert_eq!(map.is_free_pos(Pos { x: 10, y: 0 }), false);
        assert_eq!(map.is_free_pos(Pos { x: 3, y: 3 }), true);
    }

    #[test]
    fn it_can_draw_itself() {
        let map = Map::new(3, 3);

        assert_eq!(
            vec![
                DrawInstruction(Pos { x: 0, y: 0 }, "###".to_string()),
                DrawInstruction(Pos { x: 0, y: 1 }, "# #".to_string()),
                DrawInstruction(Pos { x: 0, y: 2 }, "###".to_string()),
            ],
            map.draw()
        )
    }
}
