use egui::{Button, Color32};

use crate::app::{self, gradient::Total};

#[derive(Debug, Default)]
pub struct ReportBuilder {
    count_fights: u32,
    count_wins: u32,
    count_draws: u32,
    count_losses: u32,
    accumulated_rounds: u32,
    accumulated_hits_dealt: u32,
    accumulated_damaging_hits_dealt: u32,
    accumulated_damage_dealt: u32,
    accumulated_hits_received: u32,
    accumulated_damaging_hits_received: u32,
    accumulated_damage_received: u32,
}

impl ReportBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_fight(&mut self, outcome: FightOutcome) {
        self.count_fights += 1;
        let stats = match outcome {
            FightOutcome::LeftWon(stats) => {
                self.count_wins += 1;
                stats
            }
            FightOutcome::RightWon(stats) => {
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
        self.accumulated_damaging_hits_dealt += stats.damaging_hits_dealt;
        self.accumulated_damage_dealt += stats.damage_dealt;
        self.accumulated_hits_received += stats.hits_received;
        self.accumulated_damaging_hits_received += stats.damaging_hits_received;
        self.accumulated_damage_received += stats.damage_received;
    }

    pub fn build(self) -> FightReport {
        if self.count_fights == 0 {
            return FightReport::ZERO;
        }

        let calc_prob = |count: u32| -> Total {
            let prob = 100 * count / self.count_fights;
            let prob: i8 = prob.try_into().unwrap();
            prob.try_into().unwrap()
        };
        let prob_win = calc_prob(self.count_wins);
        let prob_draw = calc_prob(self.count_draws);

        let avg_rounds = self.accumulated_rounds / self.count_fights;
        let avg_hits_dealt = self.accumulated_hits_dealt / self.count_fights;
        let avg_dmg_hits_dealt = self.accumulated_damaging_hits_dealt / self.count_fights;
        let avg_damage_dealt = if self.accumulated_damaging_hits_dealt == 0 {
            0
        } else {
            self.accumulated_damage_dealt / self.accumulated_damaging_hits_dealt
        };
        let avg_hits_received = self.accumulated_hits_received / self.count_fights;
        let avg_dmg_hits_received = self.accumulated_damaging_hits_received / self.count_fights;
        let avg_damage_received = if self.accumulated_damaging_hits_received == 0 {
            0
        } else {
            self.accumulated_damage_received / self.accumulated_damaging_hits_received
        };

        FightReport {
            prob_win,
            prob_draw,
            avg_rounds: avg_rounds.into(),
            avg_hits_dealt: avg_hits_dealt.into(),
            avg_dmg_hits_dealt: avg_dmg_hits_dealt.into(),
            avg_damage_dealt: avg_damage_dealt.into(),
            avg_hits_received: avg_hits_received.into(),
            avg_dmg_hits_received: avg_dmg_hits_received.into(),
            avg_damage_received: avg_damage_received.into(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Stat(Option<u32>);

impl From<u32> for Stat {
    fn from(value: u32) -> Self {
        Self(Some(value))
    }
}

impl Stat {
    const NONE: Self = Self(None);
    const ZERO: Self = Self(Some(0));

    fn draw(self, max_size: impl Into<egui::Vec2>, ui: &mut egui::Ui) {
        let Some(value) = self.0 else {
            ui.add_sized(max_size, egui::widgets::Spinner::new());
            return;
        };

        let dark_gray = Color32::from_rgb(64, 64, 64);
        let color = if ui.visuals().dark_mode {
            dark_gray
        } else {
            Color32::LIGHT_GRAY
        };
        let text = format!("{value}",);
        ui.add_enabled_ui(false, |ui| {
            ui.add_sized(max_size, Button::new(text).frame(false).fill(color));
        });
    }
}

#[derive(Debug, Default, Clone)]
pub struct FightReport {
    prob_win: Total,
    prob_draw: Total,
    avg_rounds: Stat,
    avg_hits_dealt: Stat,
    avg_dmg_hits_dealt: Stat,
    avg_damage_dealt: Stat,
    avg_hits_received: Stat,
    avg_dmg_hits_received: Stat,
    avg_damage_received: Stat,
}

impl FightReport {
    pub const NONE: Self = Self {
        prob_win: Total::NONE,
        prob_draw: Total::NONE,
        avg_rounds: Stat::NONE,
        avg_hits_dealt: Stat::NONE,
        avg_dmg_hits_dealt: Stat::NONE,
        avg_damage_dealt: Stat::NONE,
        avg_hits_received: Stat::NONE,
        avg_dmg_hits_received: Stat::NONE,
        avg_damage_received: Stat::NONE,
    };
    pub const ZERO: Self = Self {
        prob_win: Total::ZERO,
        prob_draw: Total::ZERO,
        avg_rounds: Stat::ZERO,
        avg_hits_dealt: Stat::ZERO,
        avg_dmg_hits_dealt: Stat::ZERO,
        avg_damage_dealt: Stat::ZERO,
        avg_hits_received: Stat::ZERO,
        avg_dmg_hits_received: Stat::ZERO,
        avg_damage_received: Stat::ZERO,
    };

    const STAT_SIZE: [f32; 2] = [30.0, 20.0];

    pub fn total(&self) -> Total {
        self.prob_win
    }

    pub fn draw(&self, ui: &mut egui::Ui) {
        const NUM_TABLES: f32 = 3.0;
        const TABLE_WIDTH: f32 = app::EDITOR_WIDTH / (NUM_TABLES * 1.1);

        fn draw_table(ui: &mut egui::Ui, id: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
            ui.group(|ui| {
                ui.set_width(TABLE_WIDTH);
                // add extra vertical() to work around grid alignment issues
                ui.vertical(|ui| {
                    egui::Grid::new(id).striped(true).show(ui, add_contents);
                });
            });
        }

        let highlight_color = if ui.visuals().dark_mode {
            egui::Color32::CYAN
        } else {
            egui::Color32::BLUE
        };

        ui.group(|ui| {
            // required for centering
            ui.set_width(app::EDITOR_WIDTH);

            ui.horizontal(|ui| {
                draw_table(ui, "results1", |ui| {
                    ui.visuals_mut().override_text_color = Some(highlight_color);
                    ui.label("Gewinnchance");
                    self.prob_win.draw(Self::STAT_SIZE, ui);
                    ui.visuals_mut().override_text_color = None;
                    ui.end_row();

                    ui.label("Chance Unentsch.");
                    self.prob_draw.draw(Self::STAT_SIZE, ui);
                    ui.end_row();

                    ui.label("Ø Runden / Kampf");
                    self.avg_rounds.draw(Self::STAT_SIZE, ui);
                    ui.end_row();
                });

                draw_table(ui, "results2", |ui| {
                    ui.label("Ø Treffer / Kampf");
                    self.avg_hits_dealt.draw(Self::STAT_SIZE, ui);
                    ui.end_row();

                    ui.label("Ø Treffer m. Schaden");
                    self.avg_dmg_hits_dealt.draw(Self::STAT_SIZE, ui);
                    ui.end_row();

                    ui.label("Ø Schaden / Schlag");
                    self.avg_damage_dealt.draw(Self::STAT_SIZE, ui);
                    ui.end_row();
                });

                draw_table(ui, "results3", |ui| {
                    ui.label("Ø erh. Treffer");
                    self.avg_hits_received.draw(Self::STAT_SIZE, ui);
                    ui.end_row();

                    ui.label("Ø erh. Treffer m. Schaden");
                    self.avg_dmg_hits_received.draw(Self::STAT_SIZE, ui);
                    ui.end_row();

                    ui.label("Ø erh. Schaden / Schlag");
                    self.avg_damage_received.draw(Self::STAT_SIZE, ui);
                    ui.end_row();
                });
            });
        });
    }
}

#[derive(Debug, Default, Clone)]
pub struct FightStats {
    rounds: u32,
    hits_dealt: u32,
    damaging_hits_dealt: u32,
    damage_dealt: u32,
    hits_received: u32,
    damaging_hits_received: u32,
    damage_received: u32,
}

impl FightStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_round(&mut self) {
        self.rounds += 1;
    }

    pub fn add_hits_dealt(&mut self, count: u8) {
        self.hits_dealt += u32::from(count);
    }

    pub fn add_damage_dealt(&mut self, damage: u8) {
        self.damaging_hits_dealt += 1;
        self.damage_dealt += u32::from(damage);
    }

    pub fn add_hits_received(&mut self, count: u8) {
        self.hits_received += u32::from(count);
    }

    pub fn add_damage_received(&mut self, damage: u8) {
        self.damaging_hits_received += 1;
        self.damage_received += u32::from(damage);
    }
}

#[derive(Debug, Clone)]
pub enum FightOutcome {
    LeftWon(FightStats),
    RightWon(FightStats),
    Draw(FightStats),
}
