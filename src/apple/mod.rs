use crate::common::{Pos, Size};
use crate::game::{AppleTrait, Draw, DrawInstruction, IsFreePos, PositionGet, Spawn};
use rand::{thread_rng, Rng};

pub struct Apple {
    pub pos: Pos,
}

impl Apple {
    pub fn new() -> Apple {
        Apple {
            pos: Pos { x: 0, y: 0 },
        }
    }
}

impl<T: IsFreePos> AppleTrait<T> for Apple {}

impl<T: IsFreePos> Spawn<T> for Apple {
    /// Initialize Apple in a random place
    fn spawn(&mut self, boundary: &Size, position_checker: &T) {
        let mut x: usize;
        let mut y: usize;

        while {
            x = thread_rng().gen_range(1, boundary.x - 2);
            y = thread_rng().gen_range(1, boundary.y - 2);
            !position_checker.is_free_pos(Pos { x, y })
        } {}

        self.pos = Pos { x, y }
    }
}

impl PositionGet for Apple {
    fn get_pos(&self) -> &Pos {
        return &self.pos;
    }
}

impl Draw for Apple {
    fn draw(&self) -> Vec<DrawInstruction> {
        let pos = self.pos.clone();
        let figure = "♥".to_string();
        vec![DrawInstruction(pos, figure)]
    }
}

#[cfg(test)]
mod test {

    use super::Apple;
    use crate::common::{Size};
    use crate::game::*;

    #[test]
    fn it_can_spawn_apple() {
        // GIVEN
        let mut one = Apple::new();
        let mut two = Apple::new();
        let mut mock_is_free_pos = MockIsFreePos::new();
        let size = Size { x: 10, y: 10 };

        mock_is_free_pos.expect_is_free_pos().returning(|_| true);

        // WHEN
        one.spawn(&size, &mock_is_free_pos);

        while one.get_pos() == two.get_pos() {
            two.spawn(&size, &mock_is_free_pos);
        }

        // THEN
        assert_ne!(one.get_pos(), two.get_pos());
    }

    #[test]
    fn it_can_draw_itself() {
        let apple = Apple::new();
        let paint = apple.draw();

        assert_eq!(paint.len(), 1);
        assert_eq!(&paint[0].0, apple.get_pos());
        assert_eq!(paint[0].1, "♥".to_string());
    }
}
