pub mod entities {
    pub type EntityID = usize;
    pub type ComponentList<T> = Vec<(EntityID,T)>;
}
