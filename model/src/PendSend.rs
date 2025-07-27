pub trait PendSend<C,M>{
    pub async fn first_pend(conn: PoolConnection<C>) -> model::Result<Option<M>>;
    pub async fn mark_complete(self:M,conn: PoolConnection<C> ) -> model::Result<()>;
}