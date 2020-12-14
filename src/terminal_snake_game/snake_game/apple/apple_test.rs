use super::*;
use super::super::game::traits::*;

#[test]
fn it_can_spawn_apple() {
    // GIVEN
    let mut one = Apple::new();
    let mut two = Apple::new();
    let mut mock_is_free_pos = MockIsFreePos::new();
    let size = Vec2 { x: 10, y: 10 };

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
    assert_eq!(paint.first().unwrap().shape.len(), 1);
    assert_eq!(paint.first().unwrap().shape.first().unwrap().0, Direction::Up);
    assert_eq!(paint.first().unwrap().shape.first().unwrap().1, Symbol::Apple);
}
