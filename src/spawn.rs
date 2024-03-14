use std::time::Instant;

use legion::World;

use crate::*;

static HERO_MARKS : [char; 4] = 
    ['@', 'P', '`', 'p'];
static VILLAIN_MARKS : [[char; 4]; 4] = [ 
    ['!', '1', 'A', '!'],
    ['"', '2', 'B', '"'],
    ['#', '3', 'C', '#'],
    ['$', '4', 'D', '$'],
];

pub fn spawn_hero (ecs : &mut World, pos : Position) {
    ecs.push((
        Character(&HERO_MARKS),
        Hero,
        Victim,
        pos,
        Direction::Down,
        ColorPair::new(WHITE, BLACK),
    ));
}
pub fn spawn_villain(ecs : &mut World, pos : Position, i: usize) {
    ecs.push((
        Character(&VILLAIN_MARKS[i % VILLAIN_MARKS.len()]),
        Villain,
        Hunter,
        RandomWalk{time: Instant::now()},
        SmartBot{time: Instant::now()},
        pos,
        Direction::Down,
        ColorPair::new(WHITE, BLACK),
    ));
}
pub fn spawn_seed(ecs : &mut World, pos : Position) {
    ecs.push((
        Food('.'),
        pos,
    ));
}
pub fn spawn_powerup(ecs : &mut World, pos : Position) {
    ecs.push((
        Food('*'),
        pos,
        Powerup,
    ));
}