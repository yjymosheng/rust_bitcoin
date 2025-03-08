impl < __Context > :: bincode :: Decode < __Context > for BlockHeader
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        Ok(Self
        {
            version : :: bincode :: Decode :: decode(decoder) ?, timestamp :
            :: bincode :: Decode :: decode(decoder) ?, tx_hash : :: bincode ::
            Decode :: decode(decoder) ?, difficulty_target : :: bincode ::
            Decode :: decode(decoder) ?, pre_hash : :: bincode :: Decode ::
            decode(decoder) ?, nonce : :: bincode :: Decode :: decode(decoder)
            ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for BlockHeader
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        Ok(Self
        {
            version : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, timestamp : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, tx_hash : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, difficulty_target : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?,
            pre_hash : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, nonce : :: bincode :: BorrowDecode ::<
            '_, __Context >:: borrow_decode(decoder) ?,
        })
    }
}