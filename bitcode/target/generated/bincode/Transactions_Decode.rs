impl < __Context > :: bincode :: Decode < __Context > for Transactions
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    { Ok(Self { transactions : :: bincode :: Decode :: decode(decoder) ?, }) }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for Transactions
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        Ok(Self
        {
            transactions : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}