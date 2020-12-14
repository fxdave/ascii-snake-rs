use super::*;

#[test]
fn it_can_make_a_new_map() {
    let map = Map::new(20, 20);
    assert_eq!(map.content.len(), 20);
    assert_eq!(map.content.get(0).unwrap().len(), 20);
}

#[test]
fn it_can_check_collision() {
    let map = Map::new(20, 20);
    assert_eq!(map.is_free_pos(&Vec2 { x: 10, y: 0 }), false);
    assert_eq!(map.is_free_pos(&Vec2 { x: 3, y: 3 }), true);
}

#[test]
fn it_can_draw_itself() {
    let map = Map::new(3, 3);

    assert_eq!(
        vec![
            DrawInstruction { 
                pos: Vec2 { x: 0, y: 0 },
                shape: vec![
                    Directed(Direction::Up, Symbol::Wall),
                    Directed(Direction::Up, Symbol::Wall),
                    Directed(Direction::Up, Symbol::Wall),
                ]
            },
            DrawInstruction { 
                pos: Vec2 { x: 0, y: 1 },
                shape: vec![
                    Directed(Direction::Up, Symbol::Wall),
                    Directed(Direction::Up, Symbol::Empty),
                    Directed(Direction::Up, Symbol::Wall),
                ]
            },
            DrawInstruction { 
                pos: Vec2 { x: 0, y: 2 },
                shape: vec![
                    Directed(Direction::Up, Symbol::Wall),
                    Directed(Direction::Up, Symbol::Wall),
                    Directed(Direction::Up, Symbol::Wall),
                ]
            },
        ],
        map.draw()
    )
}
