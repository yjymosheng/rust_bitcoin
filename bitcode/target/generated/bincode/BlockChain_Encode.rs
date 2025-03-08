impl :: bincode :: Encode for BlockChain
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.blocks, encoder) ?; :: bincode ::
        Encode :: encode(&self.difficulty, encoder) ?; Ok(())
    }
}