impl :: bincode :: Encode for BlockHeader
{
    fn encode < __E : :: bincode :: enc :: Encoder >
    (& self, encoder : & mut __E) ->core :: result :: Result < (), :: bincode
    :: error :: EncodeError >
    {
        :: bincode :: Encode :: encode(&self.version, encoder) ?; :: bincode
        :: Encode :: encode(&self.timestamp, encoder) ?; :: bincode :: Encode
        :: encode(&self.tx_hash, encoder) ?; :: bincode :: Encode ::
        encode(&self.difficulty_target, encoder) ?; :: bincode :: Encode ::
        encode(&self.pre_hash, encoder) ?; :: bincode :: Encode ::
        encode(&self.nonce, encoder) ?; Ok(())
    }
}