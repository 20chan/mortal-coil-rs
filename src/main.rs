extern crate reqwest;
use reqwest::Client;
use std::fmt;

fn main() {
    let client = Client::new();
    login(&client);

    let (level, board) = get_board(&client).expect("ang you failed idiot");
    println!("Level: {}", level);
    println!("{:?}", board);
}

const url: &str = "http://www.hacker.org/coil/index.php";

fn login(client: &Client) {
    let data = "username=rust_rulz&password=realtrue&redirect=&login=Log+in";
    let resp = client
        .post("http://www.hacker.org/forum/login.php")
        .header(reqwest::header::HOST, "www.hacker.org")
        .header(
            reqwest::header::CONTENT_TYPE,
            "application/x-www-form-urlencoded",
        ).body(data)
        .send()
        .unwrap();

    let headers = resp.headers();

    println!("{:?}", headers);
}

fn get_board(client: &Client) -> Option<(usize, Board)> {
    let body = client.get(url).send().unwrap().text().unwrap();

    let r = body.split("Level: ").nth(1)?;
    let l = r.split('<').nth(0).unwrap();
    let level = l.parse().unwrap();
    let r = body.split("FlashVars=\"").collect::<Vec<_>>()[1];
    let l = r.split('"').collect::<Vec<_>>()[0];
    let board = parse_board(l);

    Some((level, board))
}

fn submit_board(client: &Client, x: usize, y: usize, path: Vec<Dir>) -> bool {
    let res = client
        .get(
            format!(
                "{}?x={}&y={}&path={}",
                url,
                x,
                y,
                path.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            ).parse::<reqwest::Url>()
            .unwrap(),
        ).send()
        .unwrap()
        .text()
        .unwrap();

    !res.starts_with("<br>")
}

fn parse_board(data: &str) -> Board {
    let parameters = data
        .split('&')
        .map(|s| s.split('=').collect::<Vec<_>>()[1])
        .collect::<Vec<_>>();
    let x = parameters[0].parse().unwrap();
    let y = parameters[1].parse().unwrap();
    let board_data = parameters[2].chars().collect::<Vec<_>>();

    let mut board = Vec::with_capacity(y + 1);
    for i in 0..y {
        board.push(Vec::with_capacity(x + 1));

        for j in 0..x {
            board[i].push(Square::new(board_data[i * x + j]).unwrap());
        }
    }

    board
}

fn try_solve(x: usize, y: usize, board: &Board) -> Option<Vec<Dir>> {
    None
}

#[derive(Debug)]
enum Dir {
    U,
    D,
    R,
    L,
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

type Board = Vec<Vec<Square>>;

#[derive(Debug)]
enum Square {
    Blank,
    Wall,
}

impl Square {
    fn new(ch: char) -> Option<Self> {
        match ch {
            '.' => Some(Square::Blank),
            'X' => Some(Square::Wall),
            _ => None,
        }
    }
}
