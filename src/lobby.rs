use std::collections::HashMap;

use crate::domain::{AbstractLobby, Game, Observer, SetTicketError, UserId, Wish, Id};

struct Lobby<G: Game, O: Observer<G>> {
    tickets: HashMap<UserId, (G::Wish, O)>,
    game_counter: G::GameId,
}

impl<G, O> AbstractLobby<G, O> for Lobby<G, O>
where
    G: Game,
    O: Observer<G>,
{
    fn new() -> Lobby<G, O> {
        Lobby {
            tickets: HashMap::new(),
            game_counter: G::GameId::new(),
        }
    }
    fn add_ticket(
        &mut self,
        user: UserId,
        new_wish: G::Wish,
        new_observer: O,
    ) -> Result<(), SetTicketError> {

        let mut paired = false;
        let mut paired_user = Option::None;
        for ticket in &self.tickets {
            let (wish, observer) = ticket.1;
            let user_id = *ticket.0;

            if wish.is_match(&new_wish) {
                paired = true;
                paired_user.replace(user_id);
                observer.notify_game(self.game_counter);
                new_observer.notify_game(self.game_counter);
                break
            }
        };
        if paired {
            let _ = self.tickets.remove(&paired_user.unwrap());
            Ok(())
        } else {
            match self.tickets.insert(user, (new_wish, new_observer)) {
                None => Ok(()),
                Some(_) => Err(SetTicketError::DuplicateTicket),
            }
        }
    }
}
