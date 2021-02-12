use crate::*;

/// Tags this entity as a `Tower`.
pub struct Tower;
/// Allows this entity to attack other entities in proximity to it.
#[derive(new)]
pub enum ProximityAttackSystems {
    /// Standard proximity attack
    SimpleProximityAttack(f32),
    /// Leader proximity attack
    Leader1ProximityAttack(f32),
}
/// Identifies which type is the companion
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Companion {
    /// A elephant companion
    Elephant(Entity),
}
/// Tags a tower projectile.
pub struct TowerProjectile;
/// Tags a core.
pub struct Core;
/// Tags a barrack.
pub struct Barrack;
/// Identifies an entity as a leader with a its position in the teams selected leaders.
pub struct Leader(pub u8);
/// Adds a name to an entity.
pub struct Name(pub String);
/// Allows this entity to move to the closest enemy entity.
pub enum MovementSystems {
    /// Standard movement
    SimpleMovement,
    /// Melee leader movement
    Leader1SimpleMovement,
    /// Ranged leader movement
    Leader2SimpleMovement,
}
/// Makes this entity run back to its team's `Core` when low in health.
pub struct FleeToBase(pub f64);
/// Added on entities which temporarily cannot move.
pub struct IsCaught(pub bool);
/// Tracks whether or not Spell Steal has been used for heroes with that ability.
pub struct SpellSteal(pub bool);
/// Tags a pawn.
pub struct Pawn;
/// Tags a pawn spawner. Contains the delay in ticks between spawns.
pub struct PawnSpawner(pub u32, pub u32);
/// Tags a base.
pub struct Base;
/// Identifies mouse selectable entities
pub struct MouseSelectable;
/// Identifies mouse clickable entities
pub struct MouseClickable;
/// Identifies mouse hoverable entities
pub struct MouseHoverable;
/// Tags an arbitrary entity spawner. Unused.
pub struct Spawner<F: Fn(&mut World)> {
    /// The spawning function.
    pub f: F,
}
/// Tags the player.
pub struct Player;

/// Identifies in which team this entity is in.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Team {
    /// Our team.
    Me,
    /// The opponent's team.
    Other,
}

/// Allows a unit to see others.
#[derive(new)]
pub struct LineOfSight {
    /// The limit of the unit vision.
    pub range: i32,
}
