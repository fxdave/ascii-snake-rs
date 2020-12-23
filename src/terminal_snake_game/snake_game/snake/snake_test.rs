use super::*;

#[test]
fn it_can_get_head() {
    let snake = Snake {
        body: linked_list! {
            Directed(Direction::Right, Vec2 { x: 3, y: 3 }),
            Directed(Direction::Right, Vec2 { x: 4, y: 3 }),
            Directed(Direction::Down, Vec2 { x: 5, y: 3 }),
            Directed(Direction::Down, Vec2 { x: 5, y: 4 }),
        },
        growing: false,
    };

    assert_eq!(snake.get_head().1, Vec2 { x: 5, y: 4 });
}

#[test]
fn it_can_get_head_pos() {
    let snake = Snake {
        body: linked_list! {
            Directed(Direction::Right, Vec2 { x: 3, y: 3 }),
            Directed(Direction::Right, Vec2 { x: 4, y: 3 }),
            Directed(Direction::Down, Vec2 { x: 5, y: 3 }),
            Directed(Direction::Down, Vec2 { x: 5, y: 4 }),
        },
        growing: false,
    };

    assert_eq!(snake.get_head_pos().x, 5);
    assert_eq!(snake.get_head_pos().y, 4);
}

#[test]
fn it_can_detect_self_eating_step() {
    let snake = Snake {
        body: linked_list! {
            Directed(Direction::Right, Vec2 { x: 3, y: 3 }),
            Directed(Direction::Right, Vec2 { x: 4, y: 3 }),
            Directed(Direction::Down, Vec2 { x: 5, y: 3 }),
            Directed(Direction::Down, Vec2 { x: 5, y: 4 }),
        },
        growing: false,
    };

    assert!(snake.is_self_eating_step(&Directed(Direction::Down, Vec2 { x: 5, y: 3 })));
    assert!(!snake.is_self_eating_step(&Directed(Direction::Down, Vec2 { x: 3, y: 5 })));
}

#[test]
fn it_can_step() {
    let mut snake = Snake::new();

    let pos_initial = Vec2 {
        x: snake.get_head_pos().x,
        y: snake.get_head_pos().y,
    };

    if let Err(SelfEatingStepError) = snake.step() {
        unreachable!( "initial step shouldn't be self eating");
    }

    let pos_after = Vec2 {
        x: snake.get_head_pos().x,
        y: snake.get_head_pos().y,
    };

    assert_ne!(pos_initial, pos_after);
}

#[test]
fn it_can_draw_itself() {
    let snake = Snake {
        body: linked_list! {
            Directed(Direction::Right, Vec2 { x: 3, y: 3 }),
            Directed(Direction::Right, Vec2 { x: 4, y: 3 }),
            Directed(Direction::Down, Vec2 { x: 5, y: 3 }),
            Directed(Direction::Down, Vec2 { x: 5, y: 4 }),
        },
        growing: false,
    };

    assert_eq!(
        vec![
            DrawInstruction{
                pos: Vec2 { x: 3, y: 3 }, 
                shape: vec![Directed(Direction::Right, Symbol::SnakeBody)]
            },
            DrawInstruction{
                pos: Vec2 { x: 4, y: 3 }, 
                shape: vec![Directed(Direction::Right, Symbol::SnakeBody)]
            },
            DrawInstruction{
                pos: Vec2 { x: 5, y: 3 }, 
                shape: vec![Directed(Direction::Down, Symbol::SnakeBody)]
            },
            DrawInstruction{
                pos: Vec2 { x: 5, y: 4 }, 
                shape: vec![Directed(Direction::Down, Symbol::SnakeHead)]
            },
        ],
        snake.draw()
    )
}
