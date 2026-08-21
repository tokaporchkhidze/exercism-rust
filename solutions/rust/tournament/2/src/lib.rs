use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(Debug)]
struct Stats<'a> {
    team: &'a str,
    played: u32,
    win: u32,
    loss: u32,
    draw: u32,
    points: u32,
}

impl Stats<'_> {
    fn increment_stats(&mut self, win: u32, loss: u32, draw: u32, points: u32) {
        self.played += 1;
        self.win += win;
        self.loss += loss;
        self.draw += draw;
        self.points += points;
    }
}

impl From<&Stats<'_>> for String {
    fn from(stats: &Stats) -> Self {
        format!(
            "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
            stats.team, stats.played, stats.win, stats.draw, stats.loss, stats.points
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let mut team_stats: HashMap<&str, Stats> = HashMap::new();
    for line in match_results.lines() {
        let tokens: Vec<&str> = line.split(';').collect();
        if tokens.len() != 3 {
            panic!("Invalid input line");
        }
        let (home_team, away_team, result) = (tokens[0], tokens[1], tokens[2]);
        let (
            home_win,
            home_loss,
            home_draw,
            home_points,
            away_win,
            away_loss,
            away_draw,
            away_points,
        ) = match result {
            "win" => (1, 0, 0, 3, 0, 1, 0, 0),
            "loss" => (0, 1, 0, 0, 1, 0, 0, 3),
            "draw" => (0, 0, 1, 1, 0, 0, 1, 1),
            _ => unreachable!(),
        };
        team_stats
            .entry(home_team)
            .or_insert(Stats {
                team: home_team,
                played: 0,
                win: 0,
                loss: 0,
                draw: 0,
                points: 0,
            })
            .increment_stats(home_win, home_loss, home_draw, home_points);
        team_stats
            .entry(away_team)
            .or_insert(Stats {
                team: away_team,
                played: 0,
                win: 0,
                loss: 0,
                draw: 0,
                points: 0,
            })
            .increment_stats(away_win, away_loss, away_draw, away_points);
    }
    let mut score_table: Vec<String> = Vec::with_capacity(team_stats.len() + 1);
    score_table.push(format!(
        "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
        "Team", "MP", "W", "D", "L", "P"
    ));
    let mut sorted_teams: Vec<&Stats> = team_stats.values().collect();
    sorted_teams.sort_by_key(|stats| (Reverse(stats.points), stats.team));
    score_table.extend(sorted_teams.into_iter().map(|stats| stats.into()));
    score_table.join("\n")
}
