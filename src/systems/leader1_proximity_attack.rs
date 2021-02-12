use crate::*;
use rand::thread_rng;
use rand::Rng;

/// Attacks entities that are close to this leader.
pub fn leader1_proximity_attack_system(
    entities: &Entities,
    proximity_attacks: &Components<ProximityAttackSystems>,
    leaders: &Components<Leader>,
    teams: &Components<Team>,
    positions: &Components<Point>,
    stats: &mut Components<StatSet<Stats>>,
    is_caught: &mut Components<IsCaught>,
    game_events: &mut Vec<GameEvent>,
) -> SystemResult {
    let mut v = vec![];
    let mut rng = thread_rng();
    for (e, proximity, stat, pos, team) in
        join!(&entities && &proximity_attacks && &stats && &positions && &teams)
    {
        if let ProximityAttackSystems::Leader1ProximityAttack(radius) = proximity.unwrap() {
            let closest = find_closest_in_other_team(
                team.unwrap(),
                pos.unwrap(),
                &teams,
                &positions,
                &stats,
                &entities,
            );
            if let Some((target, p)) = closest {
                if dist(&p, pos.unwrap()) <= *radius {
                    let damage = stat.unwrap().stats.get(&Stats::Attack).unwrap().value;
                    v.push((e.unwrap().clone(), target.clone(), damage));
                }
            }
        }
    }
    // 5% chance of getting caught if leaders are in range of each other
    for (proximity, pos, team, _, mut caught) in
        join!(&proximity_attacks && &positions && &teams && &leaders && &mut is_caught)
    {
        if let ProximityAttackSystems::Leader1ProximityAttack(radius) = proximity.unwrap() {
            let mut vec = join!(&entities && &teams && &positions && &stats && &leaders)
                .filter(|(_e, t, _, _, _)| *t.unwrap() != *team.unwrap())
                .map(|(e, _, p, _, _)| (dist(pos.unwrap(), p.unwrap()), e.unwrap()))
                .filter(|(d, _)| *d < *radius)
                .collect::<Vec<_>>();
            vec.sort_by(|e1, e2| e1.0.partial_cmp(&e2.0).unwrap());
            let closest = vec.into_iter().next().map(|(_d, p)| p);
            if let Some(_) = closest {
                // 5% chance of leaders getting caught
                if rng.gen_range(1, 21) == 1 {
                    caught.as_mut().unwrap().0 = true;
                }
            }
        }
    }

    for (attacker, target, dmg) in v.into_iter() {
        increment_attacks_dealt(&mut stats.get_mut(attacker).unwrap());
        increment_attacks_received(&mut stats.get_mut(target).unwrap());
        game_events.push(GameEvent::DamageEntity(attacker, target, dmg));
    }
    Ok(())
}
