use std::collections::HashMap;
use crate::mapping::{GrowableMapping, Mapping, RankMapping};
use std::convert::Infallible;
use std::error::Error;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;


/// Trait that has to be implemented on types that want to be extra information for each
/// element of a [`GenericUnionFind`](crate::generic::UnionFind).
pub trait Extra<K, V> {
    type DefaultMappingErr: Error;

    fn default_mapping(elems: impl IntoIterator<Item = K>) -> Result<Self, Self::DefaultMappingErr>
    where
        Self: Sized;
}

/// () trivially implements Extra, which is the default when there is no extra info.
impl<K, V> Extra<K, V> for () {
    type DefaultMappingErr = Infallible;

    fn default_mapping(_elems: impl IntoIterator<Item = K>) -> Result<Self, Self::DefaultMappingErr>
    where
        Self: Sized,
    {
        Ok(())
    }
}

pub trait GrowableExtra<K, V> {
    type AddError: Error + Debug;

    fn add(&mut self, k: K, v: V) -> Result<(), Self::AddError>
    where
        Self: Sized;
}

/// () trivially implements GrowableExtra, which is the default when there is no extra info.
impl<K, V> GrowableExtra<K, V> for () {
    type AddError = Infallible;

    fn add(&mut self, _k: K, _v: V) -> Result<(), Self::AddError>
    where
        Self: Sized,
    {
        Ok(())
    }
}

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))]
pub struct ByRank<T: Hash + Eq> {
    #[serde_as(as = "Vec<(_, _)>")]
    mapping: HashMap<T, usize>,
    phantom: PhantomData<T>,
}

impl<T: Hash+Eq> ByRank<T>
{
    pub fn new(elems: impl IntoIterator<Item = T>) -> Result<Self, ()> {
        Ok(Self {
            mapping: HashMap::zero_map(elems).unwrap(),
            phantom: Default::default(),
        })
    }
}

impl<T: Hash+Eq> ByRank<T>
{
    pub fn rank(&self, elem: &T) -> Option<usize> {
        self.mapping.get(elem).cloned()
    }

    pub fn set_rank(&mut self, elem: T, rank: usize) {
        self.mapping.set(elem, rank)
    }
}

impl<T: Hash+Eq> Extra<T, usize> for ByRank<T>
{
    type DefaultMappingErr = <HashMap<T, usize> as RankMapping<T>>::Err;

    fn default_mapping(
        elems: impl IntoIterator<Item = T>,
    ) -> Result<Self, Self::DefaultMappingErr> {
        Ok(Self {
            mapping: HashMap::zero_map(elems)?,
            phantom: Default::default(),
        })
    }
}

impl<T: Hash+ Eq> GrowableExtra<T, usize> for ByRank<T>
{
    type AddError = <HashMap<T, usize> as GrowableMapping<T, usize>>::AddError;

    fn add(&mut self, elem: T, value: usize) -> Result<(), Self::AddError> {
        self.mapping.add(elem, value)
    }
}
