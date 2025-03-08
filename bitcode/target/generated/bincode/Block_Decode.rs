impl < __Context > :: bincode :: Decode < __Context > for Block
{
    fn decode < __D : :: bincode :: de :: Decoder < Context = __Context > >
    (decoder : & mut __D) ->core :: result :: Result < Self, :: bincode ::
    error :: DecodeError >
    {
        Ok(Self
        {
            header : :: bincode :: Decode :: decode(decoder) ?, transactions :
            :: bincode :: Decode :: decode(decoder) ?, hash : :: bincode ::
            Decode :: decode(decoder) ?,
        })
    }
} impl < '__de, __Context > :: bincode :: BorrowDecode < '__de, __Context >
for Block
{
    fn borrow_decode < __D : :: bincode :: de :: BorrowDecoder < '__de,
    Context = __Context > > (decoder : & mut __D) ->core :: result :: Result <
    Self, :: bincode :: error :: DecodeError >
    {
        Ok(Self
        {
            header : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?, transactions : :: bincode ::
            BorrowDecode ::< '_, __Context >:: borrow_decode(decoder) ?, hash
            : :: bincode :: BorrowDecode ::< '_, __Context >::
            borrow_decode(decoder) ?,
        })
    }
}