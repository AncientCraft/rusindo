use std::collections::HashMap;

struct Route
{
    handler: fn(T) -> Vec<u8>
}

pub trait ServiceFactory<Req>