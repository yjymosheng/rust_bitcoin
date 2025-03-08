impl :: bincode :: Encode for Block
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.header, encoder) ?; :: bincode ::
        Encode :: encode(&self.transactions, encoder) ?; :: bincode :: Encode
        :: encode(&self.hash, encoder) ?; Ok(())
    }
}