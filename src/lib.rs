use rand::{thread_rng, prelude::SliceRandom};
use std::{collections::VecDeque, net::TcpStream, mem, io::{Write, BufReader, Read, BufRead}};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    game: Option<Game>,
    from: Option<Player>,
    response: Response,
    card: Option<Card>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
struct Response {
    text: Option<String>,
    kind: ResponseKind,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
enum ResponseKind {
    Error,
    Success,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    players: Vec<Player>,
    deck: VecDeque<Card>,
    current_card: Card,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    name: String,
    turn: usize,
    cards: Vec<Card>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
enum CardKind {
    WildCard,
    DrawFour,
    DrawTwo,
    Cancel,
    Reverse,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    color: Color,
    kind: CardKind,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Wild,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game { 
            players: Vec::new(), 
            deck: Card::random_deck(), 
            // Set current card to placeholder
            current_card: Card { color: Color::Wild, kind: CardKind::Cancel }, 
        };

        // Set current card
        game.current_card = game.card_from_deck_top().unwrap();

        game
    }

    fn card_from_deck_top(&mut self) -> Option<Card> {
        self.deck.pop_front()
    }
    fn reshuffle(&mut self) {
        let mut deck: Vec<Card> = self.deck.clone().into();

        deck.shuffle(&mut thread_rng());

        self.deck = deck.into();
    }
    pub fn add_player(&mut self, player: &Player) {
        self.players.push(player.clone());
    }
    pub fn draw_hand(&mut self) -> Vec<Card> {
        let mut hand = Vec::with_capacity(7);

        for _ in 0..7 {
            hand.push(self.card_from_deck_top().unwrap());
        }

        hand
    }

    pub fn current_card(&self) -> &Card {
        &self.current_card
    }

    pub fn card_matches(&self, card: &Card) -> bool {
        if card.color == Color::Wild {
            return true;
        }

        card.color == self.current_card.color || card.kind == self.current_card.kind
    }
}

impl Card {
    /// Generate a random deck, uno decks consist of the following:
    /// 4 1's, 4 2's, 4 3's, etc (all of different colors, Red, Blue, Green, and Yellow)
    /// 4 Draw 2's, 4 Cancel's, 4 Reverse's, 2 Draw 4's, 2 Wild Card's, and 4 0's (all of different colors)
    /// Deck is shuffled
    fn random_deck() -> VecDeque<Card> {
        // Create a deck with all cards
        let mut deck = vec![
            Card { color: Color::Red, kind: CardKind::Reverse }, Card { color: Color::Red, kind: CardKind::DrawTwo }, Card { color: Color::Red, kind: CardKind::Cancel }, Card { color: Color::Red, kind: CardKind::Zero }, Card { color: Color::Red, kind: CardKind::One }, Card { color: Color::Red, kind: CardKind::Two }, Card { color: Color::Red, kind: CardKind::Three }, Card { color: Color::Red, kind: CardKind::Four }, Card { color: Color::Red, kind: CardKind::Five }, Card { color: Color::Red, kind: CardKind::Six }, Card { color: Color::Red, kind: CardKind::Seven }, Card { color: Color::Red, kind: CardKind::Eight }, Card { color: Color::Red, kind: CardKind::Nine },
            Card { color: Color::Blue, kind: CardKind::Reverse }, Card { color: Color::Blue, kind: CardKind::DrawTwo }, Card { color: Color::Blue, kind: CardKind::Cancel }, Card { color: Color::Blue, kind: CardKind::Zero }, Card { color: Color::Blue, kind: CardKind::One }, Card { color: Color::Blue, kind: CardKind::Two }, Card { color: Color::Blue, kind: CardKind::Three }, Card { color: Color::Blue, kind: CardKind::Four }, Card { color: Color::Blue, kind: CardKind::Five }, Card { color: Color::Blue, kind: CardKind::Six }, Card { color: Color::Blue, kind: CardKind::Seven }, Card { color: Color::Blue, kind: CardKind::Eight }, Card { color: Color::Blue, kind: CardKind::Nine },
            Card { color: Color::Green, kind: CardKind::Reverse }, Card { color: Color::Green, kind: CardKind::DrawTwo }, Card { color: Color::Green, kind: CardKind::Cancel }, Card { color: Color::Green, kind: CardKind::Zero }, Card { color: Color::Green, kind: CardKind::One }, Card { color: Color::Green, kind: CardKind::Two }, Card { color: Color::Green, kind: CardKind::Three }, Card { color: Color::Green, kind: CardKind::Four }, Card { color: Color::Green, kind: CardKind::Five }, Card { color: Color::Green, kind: CardKind::Six }, Card { color: Color::Green, kind: CardKind::Seven }, Card { color: Color::Green, kind: CardKind::Eight }, Card { color: Color::Green, kind: CardKind::Nine },
            Card { color: Color::Yellow, kind: CardKind::Reverse }, Card { color: Color::Yellow, kind: CardKind::DrawTwo }, Card { color: Color::Yellow, kind: CardKind::Cancel }, Card { color: Color::Yellow, kind: CardKind::Zero }, Card { color: Color::Yellow, kind: CardKind::One }, Card { color: Color::Yellow, kind: CardKind::Two }, Card { color: Color::Yellow, kind: CardKind::Three }, Card { color: Color::Yellow, kind: CardKind::Four }, Card { color: Color::Yellow, kind: CardKind::Five }, Card { color: Color::Yellow, kind: CardKind::Six }, Card { color: Color::Yellow, kind: CardKind::Seven }, Card { color: Color::Yellow, kind: CardKind::Eight }, Card { color: Color::Yellow, kind: CardKind::Nine },
            Card { color: Color::Wild, kind: CardKind::DrawFour}, Card { color: Color::Wild, kind: CardKind::DrawFour}, Card { color: Color::Wild, kind: CardKind::WildCard }, Card { color: Color::Wild, kind: CardKind::WildCard },
        
            Card { color: Color::Red, kind: CardKind::Reverse }, Card { color: Color::Red, kind: CardKind::DrawTwo }, Card { color: Color::Red, kind: CardKind::Cancel }, Card { color: Color::Red, kind: CardKind::Zero }, Card { color: Color::Red, kind: CardKind::One }, Card { color: Color::Red, kind: CardKind::Two }, Card { color: Color::Red, kind: CardKind::Three }, Card { color: Color::Red, kind: CardKind::Four }, Card { color: Color::Red, kind: CardKind::Five }, Card { color: Color::Red, kind: CardKind::Six }, Card { color: Color::Red, kind: CardKind::Seven }, Card { color: Color::Red, kind: CardKind::Eight }, Card { color: Color::Red, kind: CardKind::Nine },
            Card { color: Color::Blue, kind: CardKind::Reverse }, Card { color: Color::Blue, kind: CardKind::DrawTwo }, Card { color: Color::Blue, kind: CardKind::Cancel }, Card { color: Color::Blue, kind: CardKind::Zero }, Card { color: Color::Blue, kind: CardKind::One }, Card { color: Color::Blue, kind: CardKind::Two }, Card { color: Color::Blue, kind: CardKind::Three }, Card { color: Color::Blue, kind: CardKind::Four }, Card { color: Color::Blue, kind: CardKind::Five }, Card { color: Color::Blue, kind: CardKind::Six }, Card { color: Color::Blue, kind: CardKind::Seven }, Card { color: Color::Blue, kind: CardKind::Eight }, Card { color: Color::Blue, kind: CardKind::Nine },
            Card { color: Color::Green, kind: CardKind::Reverse }, Card { color: Color::Green, kind: CardKind::DrawTwo }, Card { color: Color::Green, kind: CardKind::Cancel }, Card { color: Color::Green, kind: CardKind::Zero }, Card { color: Color::Green, kind: CardKind::One }, Card { color: Color::Green, kind: CardKind::Two }, Card { color: Color::Green, kind: CardKind::Three }, Card { color: Color::Green, kind: CardKind::Four }, Card { color: Color::Green, kind: CardKind::Five }, Card { color: Color::Green, kind: CardKind::Six }, Card { color: Color::Green, kind: CardKind::Seven }, Card { color: Color::Green, kind: CardKind::Eight }, Card { color: Color::Green, kind: CardKind::Nine },
            Card { color: Color::Yellow, kind: CardKind::Reverse }, Card { color: Color::Yellow, kind: CardKind::DrawTwo }, Card { color: Color::Yellow, kind: CardKind::Cancel }, Card { color: Color::Yellow, kind: CardKind::Zero }, Card { color: Color::Yellow, kind: CardKind::One }, Card { color: Color::Yellow, kind: CardKind::Two }, Card { color: Color::Yellow, kind: CardKind::Three }, Card { color: Color::Yellow, kind: CardKind::Four }, Card { color: Color::Yellow, kind: CardKind::Five }, Card { color: Color::Yellow, kind: CardKind::Six }, Card { color: Color::Yellow, kind: CardKind::Seven }, Card { color: Color::Yellow, kind: CardKind::Eight }, Card { color: Color::Yellow, kind: CardKind::Nine },
            Card { color: Color::Wild, kind: CardKind::DrawFour}, Card { color: Color::Wild, kind: CardKind::DrawFour}, Card { color: Color::Wild, kind: CardKind::WildCard }, Card { color: Color::Wild, kind: CardKind::WildCard },
        ];

        // Shuffle the deck
        deck.shuffle(&mut thread_rng());

        // Return the shuffled deck
        deck.into()
    }
}

impl Player {
    pub fn new() -> Player {
        Player { name: String::new(), turn: 0, cards: Vec::new() }
    }   
    pub fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }
    pub fn set_turn(&mut self, turn: usize) {
        self.turn = turn;
    }
    pub fn set_cards(&mut self, cards: Vec<Card>) {
        self.cards = cards;
    }
    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }
    pub fn id(&self) -> &usize {
        &self.turn
    }
}

impl Packet {
    pub fn new(game: &Option<Game>, from: &Option<Player>) -> Packet {
        Packet {
            game: game.clone(),
            from: from.clone(),
            card: None,
            response: Response {
                kind: ResponseKind::Success,
                text: None,
            },
        }
    }

    pub fn recieved_from(&self) -> &Option<Player> {
        &self.from
    }
    pub fn mut_recieved_from(&mut self) -> &mut Option<Player> {
        &mut self.from
    }
    pub fn game(&self) -> &Option<Game> {
        &self.game
    }
    pub fn game_mut(&mut self) -> &mut Option<Game> {
        &mut self.game
    }
    pub fn card(&self) -> &Option<Card> {
        &self.card
    }

    pub fn success(&self) -> (bool, &Option<String>) {
        if self.response.kind == ResponseKind::Success {
            (true, &None)
        } else {
            (false, &self.response.text)
        }
    }

    pub fn set_error(&mut self, text: Option<String>) {
        self.response.kind = ResponseKind::Error;
        self.response.text = text;
    }
    /// Get the player with the matching turn number
    /// Returns an option containing a refrence to the player
    pub fn get_player(&self, who: &Player) -> Option<&Player> {
        if self.game().is_none() {
            return None;
        } else {
            for player in &self.game().as_ref().unwrap().players {
                if player.turn == who.turn {
                    return Some(player);
                }
            }
            return None;
        }
    }
    pub fn get_player_mut(&mut self, who: &Player) -> Option<&mut Player> {
        if self.game().is_none() {
            return None;
        } else {
            for player in &mut self.game_mut().as_mut().unwrap().players {
                if player.turn == who.turn {
                    return Some(player);
                }
            }
            return None;
        }
    }
    pub fn set_card(&mut self, card: &Card) {
        self.card = Some(card.clone());
    }

    pub fn write(&mut self, stream: &mut TcpStream) -> Result<usize, Box<dyn std::error::Error>> {
        let data = bincode::serialize(self)?;

        let content_length = mem::size_of_val(&data[..]);

        stream.write_all(
            format!("{content_length}\r\n").as_bytes()
        )?;

        stream.write_all(&data)?;

        Ok(content_length)
    }

    pub fn read(stream: &mut TcpStream) -> Result<Packet, Box<dyn std::error::Error>> {
        let mut packet_length = String::new();
        let mut buf_reader = BufReader::new(stream);

        buf_reader.read_line(&mut packet_length)?;

        let packet_length = packet_length.trim().parse()?;

        let mut packet_bytes = Vec::new();
        packet_bytes.resize(packet_length, 0);

        buf_reader.read_exact(&mut packet_bytes)?;

        let packet = bincode::deserialize::<Packet>(&packet_bytes)?;

        Ok(packet)
    }
}