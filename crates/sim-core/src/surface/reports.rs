//! After the fight: its end is recorded, and each force reports and
//! claims (decision 0014).

use super::*;

impl World {
    // ---- after the fight ----

    /// Records the end of the fight, then each force reports and claims, the
    /// lower ID first.
    pub(super) fn end_fight(&mut self, fight: &Fight, ending: &Ending) -> EventId {
        let ended = self.record_in_fight(
            fight,
            EventKind::EngagementEnded {
                round: ending.round,
                a: fight.forces[0].id,
                b: fight.forces[1].id,
                hex: fight.hex,
                reason: ending.reason,
            },
            ending.causes.clone(),
            None,
        );
        for index in 0..2 {
            self.report_sighting(fight, index, ended);
            self.claim_victory(fight, index, ended);
        }
        ended
    }

    /// A force that sighted the enemy reports the enemy force: at the fight
    /// hex, with the ships it left afloat (or began with, if none are left),
    /// through the same error model as an air search report.
    ///
    /// A force fired on in the last round has seen the enemy's gun flashes,
    /// so it reports too, even though the fight ended before the round in
    /// which it would have counted as having sighted (decision 0014, 4).
    fn report_sighting(&mut self, fight: &Fight, index: usize, ended: EventId) {
        let force = &fight.forces[index];
        if !(force.sighted || force.fired_on_last_round) {
            return;
        }
        let enemy = &fight.forces[Fight::enemy(index)];
        let afloat = self.ships_afloat(enemy.id);
        let seen = if afloat.is_empty() {
            &enemy.began_with
        } else {
            &afloat
        };
        // The smallest kind is the most important one (decision 0012,
        // decision 9).
        let kind: ShipKind = seen
            .iter()
            .map(|ship| self.class_of(*ship).kind)
            .min()
            .expect("a force that fought had ships");
        let ship_count = u32::try_from(seen.len()).expect("few ships");

        let figures = self.ref_data.search.clone();
        let mut stream = self.rng_pair(
            fight.day,
            Some(fight.pulse),
            Subsystem::Intelligence,
            Draw::Legacy,
            EntityRef::TaskForce(force.id),
            EntityRef::TaskForce(enemy.id),
        );
        let report = build_sighting_report(
            &mut stream,
            &self.map,
            fight.hex,
            ship_count,
            kind,
            &figures,
            self.events.next_id(),
        );
        // The force that sighted holds the report first (decision 0135, 1).
        let holders = Holders::Commands {
            commands: vec![self.command_of_force(force.id)],
            key: report.received_event,
            day: fight.day,
            pulse: Some(fight.pulse),
        };
        let ApplyOutcome::Contact(outcome) = self.apply_report(
            force.side,
            &Report::Sighting(report.clone()),
            &holders,
            &figures,
        ) else {
            unreachable!("a sighting report makes or refreshes a contact");
        };
        let received = self.record_in_fight(
            fight,
            EventKind::ReportReceived {
                side: force.side,
                contact: outcome.contact,
                hex: report.estimated_hex,
                ship_count: report.estimated_ship_count,
                kind: report.estimated_kind,
                created: outcome.created,
            },
            vec![Cause::Event(ended)],
            None,
        );
        assert_eq!(
            received, report.received_event,
            "the report cites the event that records it"
        );
        self.pass_report(force.side, &Report::Sighting(report), &holders);
    }

    /// A force that fired claims as sunk every enemy ship that sank in the
    /// fight and every enemy ship afloat with fire damage at the claim level.
    fn claim_victory(&mut self, fight: &Fight, index: usize, ended: EventId) {
        let force = &fight.forces[index];
        if !force.fired_this_fight {
            return;
        }
        let enemy = &fight.forces[Fight::enemy(index)];
        let burning = self.truth.task_forces[&enemy.id]
            .ships
            .iter()
            .filter(|ship| self.truth.ships[*ship].fire_damage >= fight.figures.claimed_sunk_fire)
            .count();
        let claimed_sunk = u32::try_from(enemy.sunk_in_fight.len() + burning).expect("few ships");

        let figures = self.ref_data.search.clone();
        let claim = BattleClaim {
            hex: fight.hex,
            day: fight.day,
            pulse: fight.pulse,
            claimed_sunk,
            claimed_hits: 0,
            claimed_kind: None,
            claimed_shot_down: 0,
            claimed_on_ground: 0,
            from_air: false,
            from_bombardment: false,
            from_submarine: false,
            received_event: self.events.next_id(),
            // A surface fight's claim names no type and no unit (decision 0180, 8).
            ground_types: Vec::new(),
            by_unit: Vec::new(),
            air_types: Vec::new(),
        };
        // The force that fired holds the claim first (decision 0135, 1).
        let holders = Holders::Commands {
            commands: vec![self.command_of_force(force.id)],
            key: claim.received_event,
            day: fight.day,
            pulse: Some(fight.pulse),
        };
        let outcome = self.apply_report(
            force.side,
            &Report::BattleClaim(claim.clone()),
            &holders,
            &figures,
        );
        assert_eq!(outcome, ApplyOutcome::Claim, "a claim is stored as a claim");
        let received = self.record_in_fight(
            fight,
            EventKind::ClaimReceived {
                side: force.side,
                hex: fight.hex,
                claimed_sunk,
                claimed_hits: 0,
                claimed_kind: None,
                claimed_shot_down: 0,
                claimed_on_ground: 0,
                from_air: false,
                from_bombardment: false,
                from_submarine: false,
                ground_types: Vec::new(),
                by_unit: Vec::new(),
                air_types: Vec::new(),
            },
            vec![Cause::Event(ended)],
            None,
        );
        assert_eq!(
            received, claim.received_event,
            "the claim cites the event that records it"
        );
        self.pass_report(force.side, &Report::BattleClaim(claim), &holders);
    }

    pub(super) fn record_in_fight(
        &mut self,
        fight: &Fight,
        kind: EventKind,
        causes: Vec<Cause>,
        breakdown: Option<Breakdown>,
    ) -> EventId {
        self.events
            .record(fight.day, Some(fight.pulse), kind, causes, breakdown)
    }
}
