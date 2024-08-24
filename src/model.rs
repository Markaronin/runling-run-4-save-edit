use num::BigInt;

use crate::starcode::{self, store_int};

mod account_max_vals {
    pub const NORMAL_GAMES: usize = 190000;
    pub const NORMAL_WINS: usize = 100000;
    pub const HARD_GAMES: usize = 110000;
    pub const HARD_WINS: usize = 120000;
    pub const TOTAL_SAVES: usize = 90300000;
    pub const TOTAL_SCORE: usize = 94000000;
    pub const TOTAL_DEATHS: usize = 96000000;
    pub const BOT_2000_KILLS: usize = 150000;
    pub const ODIN_KILLS: usize = 160000;
    pub const DIABLO_KILLS: usize = 170000;
    pub const INSANE_GAMES: usize = 180000;
    pub const INSANE_WINS: usize = 190000;
    pub const BLANK_3_PLACEHOLDER: usize = 200000;
    pub const TIME_GAMES: usize = 210000;
    pub const TIME_WINS: usize = 220000;
    pub const MINIGAME_HIGH_SCORE: usize = 230000;
    pub const TIME_HIGH_SCORE: usize = 240000;
    pub const CAMERA_DISTANCE: usize = 1000;
    pub const CAMERA_ROTATION: usize = 1001;
    pub const CAMERA_ANGLE: usize = 1002;
    pub const CAMERA_FOLLOW: usize = 1003;
    pub const HIDE_TIPS: usize = 1004;
    pub const HIDE_HUD: usize = 1005;
    pub const HIDE_MINIMAP: usize = 1006;
    pub const HIDE_ENERGY_BAR: usize = 1007;
    pub const HIDE_EXPERIENCE_BAR: usize = 1008;
    pub const HIDE_MENU: usize = 1009;
    pub const WASD_MOVEMENT: usize = 1010;
    pub const INCREASE_DISTANCE_SKILL: usize = 10;
    pub const DECREASE_DISTANCE_SKILL: usize = 11;
    pub const ROTATE_RIGHT_SKILL: usize = 12;
    pub const ROTATE_LEFT_SKILL: usize = 13;
    pub const FOLLOW_RUNLING_SKILL: usize = 14;
}
#[derive(Debug, PartialEq, Eq)]
pub struct Account {
    pub normal_games: usize,
    pub normal_wins: usize,
    pub hard_games: usize,
    pub hard_wins: usize,
    pub total_saves: usize,
    pub total_score: usize,
    pub total_deaths: usize,
    pub bot_2000_kills: usize,
    pub odin_kills: usize,
    pub diablo_kills: usize,
    pub insane_games: usize,
    pub insane_wins: usize,
    pub blank_3_placeholder: usize,
    pub time_games: usize,
    pub time_wins: usize,
    pub minigame_high_score: usize,
    pub time_high_score: usize,
    pub camera_distance: usize,
    pub camera_rotation: usize,
    pub camera_angle: usize,
    pub camera_follow: usize,
    pub hide_tips: usize,
    pub hide_hud: usize,
    pub hide_minimap: usize,
    pub hide_energy_bar: usize,
    pub hide_experience_bar: usize,
    pub hide_menu: usize,
    pub wasd_movement: usize,
    pub increase_distance_skill: usize,
    pub decrease_distance_skill: usize,
    pub rotate_right_skill: usize,
    pub rotate_left_skill: usize,
    pub follow_runling_skill: usize,
}
impl Account {
    pub fn from_data(data: String) -> Self {
        let data = starcode::decrypt(data);
        let mut decompressed_string = starcode::uncompress(data);

        Self {
            follow_runling_skill: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::FOLLOW_RUNLING_SKILL,
            ),
            rotate_left_skill: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::ROTATE_LEFT_SKILL,
            ),
            rotate_right_skill: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::ROTATE_RIGHT_SKILL,
            ),
            decrease_distance_skill: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::DECREASE_DISTANCE_SKILL,
            ),
            increase_distance_skill: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::INCREASE_DISTANCE_SKILL,
            ),
            wasd_movement: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::WASD_MOVEMENT,
            ),
            hide_menu: starcode::get_int(&mut decompressed_string, account_max_vals::HIDE_MENU),
            hide_experience_bar: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::HIDE_EXPERIENCE_BAR,
            ),
            hide_energy_bar: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::HIDE_ENERGY_BAR,
            ),
            hide_minimap: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::HIDE_MINIMAP,
            ),
            hide_hud: starcode::get_int(&mut decompressed_string, account_max_vals::HIDE_HUD),
            hide_tips: starcode::get_int(&mut decompressed_string, account_max_vals::HIDE_TIPS),
            camera_follow: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::CAMERA_FOLLOW,
            ),
            camera_angle: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::CAMERA_ANGLE,
            ),
            camera_rotation: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::CAMERA_ROTATION,
            ),
            camera_distance: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::CAMERA_DISTANCE,
            ),
            time_high_score: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::TIME_HIGH_SCORE,
            ),
            minigame_high_score: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::MINIGAME_HIGH_SCORE,
            ),
            time_wins: starcode::get_int(&mut decompressed_string, account_max_vals::TIME_WINS),
            time_games: starcode::get_int(&mut decompressed_string, account_max_vals::TIME_GAMES),
            blank_3_placeholder: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::BLANK_3_PLACEHOLDER,
            ),
            insane_wins: starcode::get_int(&mut decompressed_string, account_max_vals::INSANE_WINS),
            insane_games: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::INSANE_GAMES,
            ),
            diablo_kills: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::DIABLO_KILLS,
            ),
            odin_kills: starcode::get_int(&mut decompressed_string, account_max_vals::ODIN_KILLS),
            bot_2000_kills: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::BOT_2000_KILLS,
            ),
            total_deaths: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::TOTAL_DEATHS,
            ),
            total_score: starcode::get_int(&mut decompressed_string, account_max_vals::TOTAL_SCORE),
            total_saves: starcode::get_int(&mut decompressed_string, account_max_vals::TOTAL_SAVES),
            hard_wins: starcode::get_int(&mut decompressed_string, account_max_vals::HARD_WINS),
            hard_games: starcode::get_int(&mut decompressed_string, account_max_vals::HARD_GAMES),
            normal_wins: starcode::get_int(&mut decompressed_string, account_max_vals::NORMAL_WINS),
            normal_games: starcode::get_int(
                &mut decompressed_string,
                account_max_vals::NORMAL_GAMES,
            ),
        }
    }

    pub fn to_data(&self) -> String {
        let mut data = BigInt::ZERO;

        store_int(&mut data, self.normal_games, account_max_vals::NORMAL_GAMES);
        store_int(&mut data, self.normal_wins, account_max_vals::NORMAL_WINS);
        store_int(&mut data, self.hard_games, account_max_vals::HARD_GAMES);
        store_int(&mut data, self.hard_wins, account_max_vals::HARD_WINS);
        store_int(&mut data, self.total_saves, account_max_vals::TOTAL_SAVES);
        store_int(&mut data, self.total_score, account_max_vals::TOTAL_SCORE);
        store_int(&mut data, self.total_deaths, account_max_vals::TOTAL_DEATHS);
        store_int(
            &mut data,
            self.bot_2000_kills,
            account_max_vals::BOT_2000_KILLS,
        );
        store_int(&mut data, self.odin_kills, account_max_vals::ODIN_KILLS);
        store_int(&mut data, self.diablo_kills, account_max_vals::DIABLO_KILLS);
        store_int(&mut data, self.insane_games, account_max_vals::INSANE_GAMES);
        store_int(&mut data, self.insane_wins, account_max_vals::INSANE_WINS);
        store_int(
            &mut data,
            self.blank_3_placeholder,
            account_max_vals::BLANK_3_PLACEHOLDER,
        );
        store_int(&mut data, self.time_games, account_max_vals::TIME_GAMES);
        store_int(&mut data, self.time_wins, account_max_vals::TIME_WINS);
        store_int(
            &mut data,
            self.minigame_high_score,
            account_max_vals::MINIGAME_HIGH_SCORE,
        );
        store_int(
            &mut data,
            self.time_high_score,
            account_max_vals::TIME_HIGH_SCORE,
        );
        store_int(
            &mut data,
            self.camera_distance,
            account_max_vals::CAMERA_DISTANCE,
        );
        store_int(
            &mut data,
            self.camera_rotation,
            account_max_vals::CAMERA_ROTATION,
        );
        store_int(&mut data, self.camera_angle, account_max_vals::CAMERA_ANGLE);
        store_int(
            &mut data,
            self.camera_follow,
            account_max_vals::CAMERA_FOLLOW,
        );
        store_int(&mut data, self.hide_tips, account_max_vals::HIDE_TIPS);
        store_int(&mut data, self.hide_hud, account_max_vals::HIDE_HUD);
        store_int(&mut data, self.hide_minimap, account_max_vals::HIDE_MINIMAP);
        store_int(
            &mut data,
            self.hide_energy_bar,
            account_max_vals::HIDE_ENERGY_BAR,
        );
        store_int(
            &mut data,
            self.hide_experience_bar,
            account_max_vals::HIDE_EXPERIENCE_BAR,
        );
        store_int(&mut data, self.hide_menu, account_max_vals::HIDE_MENU);
        store_int(
            &mut data,
            self.wasd_movement,
            account_max_vals::WASD_MOVEMENT,
        );
        store_int(
            &mut data,
            self.increase_distance_skill,
            account_max_vals::INCREASE_DISTANCE_SKILL,
        );
        store_int(
            &mut data,
            self.decrease_distance_skill,
            account_max_vals::DECREASE_DISTANCE_SKILL,
        );
        store_int(
            &mut data,
            self.rotate_right_skill,
            account_max_vals::ROTATE_RIGHT_SKILL,
        );
        store_int(
            &mut data,
            self.rotate_left_skill,
            account_max_vals::ROTATE_LEFT_SKILL,
        );
        store_int(
            &mut data,
            self.follow_runling_skill,
            account_max_vals::FOLLOW_RUNLING_SKILL,
        );

        starcode::encrypt(starcode::compress(data))
    }

    pub fn checksum(&self) -> usize {
        self.normal_games
            + self.normal_wins
            + self.hard_games
            + self.hard_wins
            + self.total_saves
            + self.total_score
            + self.total_deaths
            + self.bot_2000_kills
            + self.odin_kills
            + self.diablo_kills
            + self.time_games
            + self.time_wins
            + self.minigame_high_score
            + self.time_high_score
    }
}

mod runling_max_vals {
    pub const CLASS: usize = 300000;
    pub const EXPERIENCE: usize = 8100000;
    pub const ENERGY_REGENERATION: usize = 320000;
    pub const MAXIMUM_ENERGY: usize = 330000;
    pub const SPEED: usize = 340000;
    pub const SKILL_1_LEVEL: usize = 350000;
    pub const SKILL_2_LEVEL: usize = 360000;
    pub const RUNLING_LEVEL: usize = 370000;
    pub const REMAINING_POINTS: usize = 380000;
}
#[derive(Debug, PartialEq, Eq)]
pub struct Runling {
    pub class: usize,
    pub experience: usize,
    pub energy_regeneration: usize,
    pub maximum_energy: usize,
    pub speed: usize,
    pub skill_1_level: usize,
    pub skill_2_level: usize,
    pub runling_level: usize,
    pub remaining_points: usize,
}
impl Runling {
    pub fn from_data(data: String) -> Self {
        let data = starcode::decrypt(data);
        let mut decompressed_string = starcode::uncompress(data);

        Self {
            remaining_points: starcode::get_int(
                &mut decompressed_string,
                runling_max_vals::REMAINING_POINTS,
            ),
            runling_level: starcode::get_int(
                &mut decompressed_string,
                runling_max_vals::RUNLING_LEVEL,
            ),
            skill_2_level: starcode::get_int(
                &mut decompressed_string,
                runling_max_vals::SKILL_2_LEVEL,
            ),
            skill_1_level: starcode::get_int(
                &mut decompressed_string,
                runling_max_vals::SKILL_1_LEVEL,
            ),
            speed: starcode::get_int(&mut decompressed_string, runling_max_vals::SPEED),
            maximum_energy: starcode::get_int(
                &mut decompressed_string,
                runling_max_vals::MAXIMUM_ENERGY,
            ),
            energy_regeneration: starcode::get_int(
                &mut decompressed_string,
                runling_max_vals::ENERGY_REGENERATION,
            ),
            experience: starcode::get_int(&mut decompressed_string, runling_max_vals::EXPERIENCE),
            class: starcode::get_int(&mut decompressed_string, runling_max_vals::CLASS),
        }
    }

    pub fn to_data(&self) -> String {
        let mut data = BigInt::ZERO;

        store_int(&mut data, self.class, runling_max_vals::CLASS);
        store_int(&mut data, self.experience, runling_max_vals::EXPERIENCE);
        store_int(
            &mut data,
            self.energy_regeneration,
            runling_max_vals::ENERGY_REGENERATION,
        );
        store_int(
            &mut data,
            self.maximum_energy,
            runling_max_vals::MAXIMUM_ENERGY,
        );
        store_int(&mut data, self.speed, runling_max_vals::SPEED);
        store_int(
            &mut data,
            self.skill_1_level,
            runling_max_vals::SKILL_1_LEVEL,
        );
        store_int(
            &mut data,
            self.skill_2_level,
            runling_max_vals::SKILL_2_LEVEL,
        );
        store_int(
            &mut data,
            self.runling_level,
            runling_max_vals::RUNLING_LEVEL,
        );
        store_int(
            &mut data,
            self.remaining_points,
            runling_max_vals::REMAINING_POINTS,
        );

        starcode::encrypt(starcode::compress(data))
    }

    pub fn checksum(&self) -> usize {
        self.remaining_points
            + self.energy_regeneration
            + self.speed
            + self.maximum_energy
            + self.skill_1_level
            + self.skill_2_level
            + self.experience
    }
}

#[cfg(test)]
mod test {
    use std::fs::read_to_string;

    use crate::model::{Account, Runling};

    #[test]
    fn runling_test() {
        let encoded_runling = read_to_string("./data/runling-1.txt").unwrap();
        let decoded_runling = Runling::from_data(encoded_runling.clone());
        let runling = Runling {
            class: 1,
            experience: 663,
            energy_regeneration: 11,
            maximum_energy: 16,
            speed: 17,
            skill_1_level: 4,
            skill_2_level: 0,
            runling_level: 15,
            remaining_points: 0,
        };

        assert_eq!(decoded_runling, runling);
        assert_eq!(runling.to_data(), encoded_runling);
    }

    #[test]
    fn account_test() {
        let encoded_account = read_to_string("./data/account.txt").unwrap();
        let decoded_account = Account::from_data(encoded_account.clone());
        let account = Account {
            normal_games: 3,
            normal_wins: 0,
            hard_games: 0,
            hard_wins: 0,
            total_saves: 37,
            total_score: 818,
            total_deaths: 81,
            bot_2000_kills: 2,
            odin_kills: 0,
            diablo_kills: 0,
            insane_games: 0,
            insane_wins: 0,
            blank_3_placeholder: 0,
            time_games: 0,
            time_wins: 0,
            minigame_high_score: 0,
            time_high_score: 0,
            camera_distance: 72,
            camera_rotation: 90,
            camera_angle: 73,
            camera_follow: 0,
            hide_tips: 0,
            hide_hud: 0,
            hide_minimap: 0,
            hide_energy_bar: 0,
            hide_experience_bar: 0,
            hide_menu: 0,
            wasd_movement: 0,
            increase_distance_skill: 0,
            decrease_distance_skill: 0,
            rotate_right_skill: 0,
            rotate_left_skill: 0,
            follow_runling_skill: 1,
        };

        assert_eq!(decoded_account, account);
        assert_eq!(account.to_data(), encoded_account);
    }
}
