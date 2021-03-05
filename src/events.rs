use crate::*;

/// Events generated from user inputs.
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum InputEvent {
    Quit,
    MenuNorth,
    MenuWest,
    MenuEast,
    MenuSouth,
    MenuSelect,
    MenuCancel,
    SpeedToggle,
    ZoomToggle,
    GameModeToggle,
    Teleport(u8),
}

/// Events generated by the game's runtime.
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum GameEvent {
    /// A team won the game.
    GameWon(Team),
    /// Attacker entity damages the target entity by this amount.
    DamageEntity(Entity, Entity, f64),
    /// Kill the specified entity.
    KillEntity(Entity),
    /// Transfers gold from an entity to another.
    TransferGold(Entity, Entity),
    /// A gold transfer was made.
    TransferedGold(Entity, f64),
    /// A leader died.
    LeaderDied(u8),
    /// Spawn a creep at the specified position in the specified team.
    SpawnCreep(Point, Team),
    /// Spawns a leader using the `TeamLeaders`' index and a position.
    SpawnLeader(Point, u8),
    /// Set additional attack for an entity.
    AdditionalAttack(Entity, f64),
    /// Set additional defense for an entity.
    AdditionalDefense(Entity, f64),
}

/// Events generated by mouse interaction.
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MouseEvent {
    /// A selectable entity was selected.
    EntitySelected(Entity),
    /// A selectable unit was selected. Distinct from `EntitySelected` because applies only to units.
    UnitSelected(Entity),
    /// A clicable entity was clicked anywhere in the game.
    EntityClicked(Entity),
    /// A hoverable entity was hovered anywhere in the game.
    EntityHovered(Entity),
    /// A point on a map was clicked.
    PositionClicked(Point),
}
