use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Default, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub struct Team<'a> {
    name: &'a str,
    matches: usize,
    wins: usize,
    ties: usize,
    loss: usize,
    points: usize,
}

impl<'a> Team<'a> {
    pub fn new(name: &str) -> Team {
        Team { 
            name,
            matches: 0,
            wins: 0,
            ties: 0,
            loss: 0,
            points: 0,
        }
    }

    pub fn winner(&mut self){
        self.wins += 1;
        self.matches += 1;
        self.points += 3;
    }

    pub fn loser(&mut self){
        self.matches += 1;
        self.loss += 1;
    }

    pub fn draw(&mut self){
        self.matches += 1;
        self.ties += 1;
        self.points += 1;
    }

    pub fn print_score(&self) -> String {
        format!("{:30} | {:2} | {:2} | {:2} | {:2} | {:2}", 
            self.name, self.matches, self.wins, self.ties, self.loss, self.points)
    }
}

type Tournament<'a> = HashMap<String, Team<'a>>;

pub fn tally(match_results: &str) -> String {
    let mut tournament: Tournament = HashMap::new();

    for line in match_results.lines() {
        let game: Vec<&str> = line.split(";").collect();
        let mut team1 = tournament.get(game[0]).copied().unwrap_or(Team::new(game[0]));
        let mut team2 = tournament.get(game[1]).copied().unwrap_or(Team::new(game[1]));
        match game[2] {
            "win" => {
                team1.winner();
                team2.loser();
            }
            "draw" =>  {
                team1.draw();
                team2.draw();
            }
            "loss" => {
                team1.loser();
                team2.winner();
            }
            _ => println!("Error!"),
        };
        tournament.insert(game[1].to_string(), team2);
        tournament.insert(game[0].to_string(), team1);
    }

    let mut sorted: Vec<&Team> = tournament.values().collect();
    sorted.sort_by(|a, b| {
        if b.points > a.points {
            Ordering::Greater
        } else if a.points > b.points {
            Ordering::Less
        } else {
            if b.name > a.name {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    });

    let mut output = String::from(format!("{:30} | MP |  W |  D |  L |  P", "Team"));
    for line in sorted {
        output.push_str("\n");
        output.push_str(line.print_score().as_str());
    }
    output
}
