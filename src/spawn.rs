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

pub fn spawn_hero (ecs : &mut World, id: u32, pos : Position) {
    ecs.push((
        Id(id),
        Character(&HERO_MARKS),
        Hero,
        pos,
        Direction::Down,
        SpecialMode(false),
    ));
}
pub fn spawn_villain(ecs : &mut World, id: u32, pos : Position) {
    ecs.push((
        Id(id),
        Character(&VILLAIN_MARKS[id as usize % VILLAIN_MARKS.len()]),
        Villain,
        pos,
        Direction::Down,
        SpecialMode(false),
    ));
}
pub fn spawn_seed(ecs : &mut World, id: u32, pos : Position) {
    ecs.push((
        Id(id),
        Food('.'),
        pos,
    ));
}
pub fn spawn_powerup(ecs : &mut World, id: u32, pos : Position) {
    ecs.push((
        Id(id),
        Food('*'),
        Powerup,
        pos,
    ));
}