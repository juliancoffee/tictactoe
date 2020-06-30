use crossbeam_channel::Sender;

struct TicketObserver<G: Game> {
    sender_channel: Sender<G::GameId>,
}

impl<G: Game> Observer<G> for TicketObserver<G> {
    fn notify_game(&self, game: G::GameId) {
        let _ = self.sender_channel.send(game);
    }
}

