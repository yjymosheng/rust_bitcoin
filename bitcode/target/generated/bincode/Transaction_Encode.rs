impl :: bincode :: Encode for Transaction
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.sender, encoder) ?; :: bincode ::
        Encode :: encode(&self.receiver, encoder) ?; :: bincode :: Encode ::
        encode(&self.amount, encoder) ?; Ok(())
    }
}