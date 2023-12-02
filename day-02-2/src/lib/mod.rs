#[derive(Copy, Clone)]
pub enum Color {
    Red = 0,
    Blue = 1,
    Green = 2,
}

impl TryFrom<&str> for Color {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Color::Red),
            "blue" => Ok(Color::Blue),
            "green" => Ok(Color::Green),
            _ => Err("Invalid color!"),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Game {
    pub id: usize,
    pub rounds: Vec<Round>,
}

impl Game {
    pub fn get_max_cubes(&self) -> Round {
        let mut nb_cubes = [0, 0, 0];

        (0..3).for_each(|cube_color| {
            let max_cube_of_color = self
                .rounds
                .iter()
                .map(|round| round.nb_cubes[cube_color])
                .max()
                .unwrap();
            nb_cubes[cube_color] = max_cube_of_color;
        });
        Round { nb_cubes }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Round {
    nb_cubes: [usize; 3],
}

impl Round {
    fn new() -> Self {
        let nb_cubes = [0, 0, 0];
        Round { nb_cubes }
    }

    pub fn new_from_list(entries: &[(Color, usize)]) -> Self {
        let mut round = Round::new();
        entries
            .iter()
            .for_each(|(color, nb_cubes)| round.add_cubes_of_color(*color, *nb_cubes));
        round
    }

    fn add_cubes_of_color(&mut self, color: Color, nb_cubes: usize) {
        self.nb_cubes[color as usize] = nb_cubes;
    }

    fn get_power(&self) -> usize {
        self.nb_cubes.iter().product()
    }
}

fn parse_game_id(game_id_str: &str) -> usize {
    let game_id_str = game_id_str.split(' ');
    let game_id_str = game_id_str.last().unwrap();
    game_id_str.parse().unwrap()
}

fn parse_cube_entry(cube_entry_str: &str) -> (Color, usize) {
    let mut cube_entry_str = cube_entry_str.trim().split(' ');
    let nb_cubes = cube_entry_str.next().unwrap().parse().unwrap();
    let color = cube_entry_str.next().unwrap().try_into().unwrap();
    (color, nb_cubes)
}

fn parse_round_str(round_str: &str) -> Round {
    let mut round = Round::new();
    round_str
        .split(',')
        .map(parse_cube_entry)
        .for_each(|(color, nb_cubes)| round.add_cubes_of_color(color, nb_cubes));
    round
}

pub fn parse_line(line: &str) -> Game {
    let mut line = line.split(':');
    let game_id_str = line.next().unwrap();
    let game_id = parse_game_id(game_id_str);

    let rounds_str = line.next().unwrap();
    let rounds = rounds_str.split(';').map(parse_round_str).collect();

    Game {
        id: game_id,
        rounds,
    }
}

pub fn process_lines<T>(lines: T) -> i32
where
    T: Iterator<Item = String>,
{
    let games = lines.map(|line| parse_line(&line));
    let games_max_cubes = games.map(|game| game.get_max_cubes());
    let games_powers = games_max_cubes.map(|round| round.get_power());
    let result: usize = games_powers.sum();
    result as i32
}
