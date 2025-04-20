use crate::gradient::Total;

#[derive(Debug, Default)]
pub struct ReportBuilder {
    count_fights: u32,
    count_wins: u32,
    count_draws: u32,
    count_losses: u32,
    accumulated_rounds: u32,
    accumulated_hits_dealt: u32,
    accumulated_damage_dealt: u32,
    accumulated_hits_received: u32,
    accumulated_damage_received: u32,
}

impl ReportBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_fight(&mut self, outcome: FightOutcome) {
        self.count_fights += 1;
        let stats = match outcome {
            FightOutcome::FighterWon(stats) => {
                self.count_wins += 1;
                stats
            }
            FightOutcome::OpponentWon(stats) => {
                self.count_losses += 1;
                stats
            }
            FightOutcome::Draw(stats) => {
                self.count_draws += 1;
                stats
            }
        };

        self.accumulated_rounds += stats.rounds;
        self.accumulated_hits_dealt += stats.hits_dealt;
        self.accumulated_damage_dealt += stats.damage_dealt;
        self.accumulated_hits_received += stats.hits_received;
        self.accumulated_damage_received += stats.damage_received;
    }

    pub fn build(self) -> FightReport {
        let calc_prob = |count: u32| -> u32 { 100 * count / self.count_fights };
        let probability_win = calc_prob(self.count_wins);
        let probability_draw = calc_prob(self.count_draws);

        let total = probability_win + probability_draw / 2;
        let total: i8 = total.try_into().unwrap();
        let total = Total::try_from(total).unwrap();

        FightReport { total }
    }
}

#[derive(Debug, Clone)]
pub struct FightReport {
    total: Total,
}

impl FightReport {
    pub const NONE: Self = Self { total: Total::NONE };

    pub fn total(&self) -> Total {
        self.total
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        self.total.draw([40.0, 40.0], ui);
    }
}

#[derive(Debug, Default, Clone)]
pub struct FightStats {
    rounds: u32,
    hits_dealt: u32,
    damage_dealt: u32,
    hits_received: u32,
    damage_received: u32,
}

impl FightStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_round(&mut self) {
        self.rounds += 1
    }

    pub fn add_hits_dealt(&mut self, count: u8) {
        self.hits_dealt += u32::from(count);
    }

    pub fn add_damage_dealt(&mut self, damage: u8) {
        self.damage_dealt += u32::from(damage);
    }

    pub fn add_hits_received(&mut self, count: u8) {
        self.hits_received += u32::from(count);
    }

    pub fn add_damage_received(&mut self, damage: u8) {
        self.damage_received += u32::from(damage);
    }
}

#[derive(Debug, Clone)]
pub enum FightOutcome {
    FighterWon(FightStats),
    OpponentWon(FightStats),
    Draw(FightStats),
}
