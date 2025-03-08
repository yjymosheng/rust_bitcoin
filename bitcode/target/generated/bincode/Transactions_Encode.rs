impl :: bincode :: Encode for Transactions
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    { :: bincode :: Encode :: encode(&self.transactions, encoder) ?; Ok(()) }
}