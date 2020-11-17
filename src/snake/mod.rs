use crate::common::{Directed, Direction, Pos};
use crate::game::{
    DirectionSet, Draw, DrawInstruction, Grow, IsFreePos, PositionGet, Reset, SelfEatingStepError,
    SelfTurningDirectionError, SnakeTrait, Step,
};
use std::collections::LinkedList;

pub struct Snake {
    body: LinkedList<Directed<Pos>>,
    growing: bool,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body: Self::get_new_body(),
            growing: false,
        }
    }

    fn get_new_body() -> LinkedList<Directed<Pos>> {
        linked_list! {
            Directed(Direction::Right, Pos { x: 3, y: 3 }),
            Directed(Direction::Right, Pos { x: 4, y: 3 }),
            Directed(Direction::Down, Pos { x: 5, y: 3 }),
            Directed(Direction::Down, Pos { x: 5, y: 4 }),
        }
    }

    fn is_self_eating_step(&self, step: &Directed<Pos>) -> bool {
        self.body
            .iter()
            .any(|i| i.1.x == step.1.x && i.1.y == step.1.y)
    }

    fn get_head(&self) -> &Directed<Pos> {
        self.body.back().expect("The snake appears to be empty")
    }

    fn get_head_mut(&mut self) -> &mut Directed<Pos> {
        self.body.back_mut().expect("The snake appears to be empty")
    }

    fn get_next_step(&self, last: &Directed<Pos>) -> Directed<Pos> {
        match last {
            Directed(Direction::Up, pos) => Directed(
                Direction::Up,
                Pos {
                    x: pos.x,
                    y: pos.y - 1,
                },
            ),
            Directed(Direction::Left, pos) => Directed(
                Direction::Left,
                Pos {
                    x: pos.x - 1,
                    y: pos.y,
                },
            ),
            Directed(Direction::Right, pos) => Directed(
                Direction::Right,
                Pos {
                    x: pos.x + 1,
                    y: pos.y,
                },
            ),
            Directed(Direction::Down, pos) => Directed(
                Direction::Down,
                Pos {
                    x: pos.x,
                    y: pos.y + 1,
                },
            ),
        }
    }
}

impl Step for Snake {
    fn step(&mut self) -> Result<(), SelfEatingStepError> {
        match self.growing {
            // In case of growing we don't delete the tail.
            true => {
                self.growing = false;
            }

            // In normal case we delete the tail.
            // This would look like a movement.
            false => {
                let _ = self.body.pop_front();
            }
        }

        let last_step = self.get_head();
        let new_step = self.get_next_step(last_step);

        if self.is_self_eating_step(&new_step) {
            Err(SelfEatingStepError)
        } else {
            self.body.push_back(new_step);
            Ok(())
        }
    }
}

impl Grow for Snake {
    fn grow(&mut self) {
        self.growing = true
    }
}

impl DirectionSet for Snake {
    fn set_direction(&mut self, direction: Direction) -> Result<(), SelfTurningDirectionError> {
        let next_step = {
            let head = self.get_head();
            let last_step = Directed(direction.clone(), head.1.clone());
            self.get_next_step(&last_step)
        };
        match self.is_self_eating_step(&next_step) {
            true => Err(SelfTurningDirectionError),
            false => {
                self.get_head_mut().0 = direction;
                Ok(())
            }
        }
    }
}

impl PositionGet for Snake {
    fn get_pos(&self) -> &Pos {
        &self.get_head().1
    }
}

impl Draw for Snake {
    fn draw(&self) -> Vec<DrawInstruction> {
        let mut paint: Vec<DrawInstruction> = self
            .body
            .iter()
            .map(|part| DrawInstruction(part.1.clone(), "#".to_string()))
            .collect();

        let last_direction = &self.body.back().expect("Empty snake").0;
        paint.last_mut().expect("Empty snake").1 = match last_direction {
            Direction::Up => "▲",
            Direction::Left => "◄",
            Direction::Right => "►",
            Direction::Down => "▼",
        }
        .to_string();

        paint
    }
}

impl IsFreePos for Snake {
    fn is_free_pos(&self, pos: Pos) -> bool {
        self.body.iter().find(|x| x.1 == pos).is_none()
    }
}

impl Reset for Snake {
    fn reset(&mut self) {
        self.body = Self::get_new_body();
        self.growing = false;
    }
}

impl SnakeTrait for Snake {}

#[test]
fn it_can_get_head() {
    let snake = Snake {
        body: linked_list! {
            Directed(Direction::Right, Pos { x: 3, y: 3 }),
            Directed(Direction::Right, Pos { x: 4, y: 3 }),
            Directed(Direction::Down, Pos { x: 5, y: 3 }),
            Directed(Direction::Down, Pos { x: 5, y: 4 }),
        },
        growing: false,
    };

    assert_eq!(snake.get_head().1, Pos { x: 5, y: 4 });
}

#[test]
fn it_can_get_pos() {
    let snake = Snake {
        body: linked_list! {
            Directed(Direction::Right, Pos { x: 3, y: 3 }),
            Directed(Direction::Right, Pos { x: 4, y: 3 }),
            Directed(Direction::Down, Pos { x: 5, y: 3 }),
            Directed(Direction::Down, Pos { x: 5, y: 4 }),
        },
        growing: false,
    };

    assert_eq!(snake.get_pos().x, 5);
    assert_eq!(snake.get_pos().y, 4);
}

#[test]
fn it_can_detect_self_eating_step() {
    let snake = Snake {
        body: linked_list! {
            Directed(Direction::Right, Pos { x: 3, y: 3 }),
            Directed(Direction::Right, Pos { x: 4, y: 3 }),
            Directed(Direction::Down, Pos { x: 5, y: 3 }),
            Directed(Direction::Down, Pos { x: 5, y: 4 }),
        },
        growing: false,
    };

    assert!(snake.is_self_eating_step(&Directed(Direction::Down, Pos { x: 5, y: 3 })));
    assert!(snake.is_self_eating_step(&Directed(Direction::Down, Pos { x: 3, y: 5 })) == false);
}

#[test]
fn it_can_step() {
    let mut snake = Snake::new();

    let pos_initial = Pos {
        x: snake.get_pos().x,
        y: snake.get_pos().y,
    };

    match snake.step() {
        Err(SelfEatingStepError) => {
            assert!(false, "initial step shouldn't be self eating");
        }
        _ => (),
    };

    let pos_after = Pos {
        x: snake.get_pos().x,
        y: snake.get_pos().y,
    };

    assert_ne!(pos_initial, pos_after);
}

#[test]
fn it_can_draw_itself() {
    let snake = Snake {
        body: linked_list! {
            Directed(Direction::Right, Pos { x: 3, y: 3 }),
            Directed(Direction::Right, Pos { x: 4, y: 3 }),
            Directed(Direction::Down, Pos { x: 5, y: 3 }),
            Directed(Direction::Down, Pos { x: 5, y: 4 }),
        },
        growing: false,
    };

    assert_eq!(
        vec![
            DrawInstruction(Pos { x: 3, y: 3 }, "#".to_string()),
            DrawInstruction(Pos { x: 4, y: 3 }, "#".to_string()),
            DrawInstruction(Pos { x: 5, y: 3 }, "#".to_string()),
            DrawInstruction(Pos { x: 5, y: 4 }, "▼".to_string()),
        ],
        snake.draw()
    )
}
