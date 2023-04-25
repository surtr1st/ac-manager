pub enum TerminalEvent<I> {
    Input(I),
    Tick,
}
