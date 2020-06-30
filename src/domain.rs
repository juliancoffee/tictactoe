use std::str::FromStr;

pub trait Wish: FromStr {
    fn is_match(&self, other: &Self) -> bool;
}

pub trait Id: Copy {
    fn new() -> Self;
    fn inc(&mut self);
}

pub trait Game {
    type Wish: Wish;
    type GameId: Id;
}

pub type UserId = u64;

pub enum SetTicketError {
    DuplicateTicket,
}

pub trait AbstractLobby<G, O>
where
    G: Game,
    O: Observer<G>,
{
    fn new() -> Self;
    fn add_ticket(
        &mut self,
        user: UserId,
        wish: G::Wish,
        observer: O,
    ) -> Result<(), SetTicketError>;
}

pub trait Observer<G: Game> {
    fn notify_game(&self, game: G::GameId);
}
