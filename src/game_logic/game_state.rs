use super::{Player, PlayerAnswerTruco};

#[derive(Clone, PartialEq, Debug)]
pub struct Truco {
    pub is_truco: bool,
    pub truco_caller: Option<Player>,
    pub truco_value: i32,
    pub is_fold: bool,
}
impl Truco {
    pub fn update_truco_state(&mut self, action: PlayerAnswerTruco, caller: Player) {
        self.is_truco = true;
        self.truco_caller = Some(caller);
        self.update_truco_values();

        if let PlayerAnswerTruco::No = action {
            self.is_fold = true;
            if self.truco_value == 3 {
                self.truco_value -= 2;
            } else {
                self.truco_value -= 3;
            }
        }
    }
    fn update_truco_values(&mut self) {
        if self.truco_value == 1 {
            self.truco_value += 2;
        } else {
            self.truco_value += 3;
        }
    }
}