impl < __Context > :: bincode :: Decode < __Context > for Transaction
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        Ok(Self
        {
            sender : :: bincode :: Decode :: decode(decoder) ?, receiver : ::
            bincode :: Decode :: decode(decoder) ?, amount : :: bincode ::
            Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for Transaction
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        Ok(Self
        {
            sender : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, receiver : :: bincode :: BorrowDecode
            ::< '_, __Context >:: borrow_decode(decoder) ?, amount : ::
            bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}