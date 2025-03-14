use legion::World;

use crate::*;

static PLAYER_MARKS : [[char; 4]; 2] = [
    ['@', 'P', '`', 'p'], // hero
    ['!', '1', 'A', '!'], // 'villain 1'
];

pub fn spawn_player1 (ecs : &mut World, id: u32, pos : Position) {
    ecs.push((
        Id(id),
        Character(&PLAYER_MARKS[0]),
        Hero,
        pos,
        Direction::Down,
    ));
}
pub fn spawn_player2 (ecs : &mut World, id: u32, pos : Position) {
    ecs.push((
        Id(id),
        Character(&PLAYER_MARKS[1]),
        Hero,
        pos,
        Direction::Down,
    ));
}
pub fn spawn_seed(ecs : &mut World, id: u32, pos : Position) {
    ecs.push((
        Id(id),
        Food('.'),
        pos,
    ));
}
pub fn spawn_superfood(ecs : &mut World, id: u32, pos : Position) {
    ecs.push((
        Id(id),
        Food('*'),
        Superfood,
        pos,
    ));
}