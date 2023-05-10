use serde::{Deserialize, Serialize};

use crate::data_structures::DbID;
use rustc_hash::FxHashSet;

// (chromosome index, bucket index)
#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Bucket(pub u8, pub u32);

// (feature id, discrete facet ids, effect size, significance)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegEffectFacets {
    pub reo_id: DbID,
    pub feature_id: DbID,
    pub facet_ids: Vec<DbID>,
    pub effect_size: f32,
    pub significance: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegEffectData {
    pub facets: Vec<RegEffectFacets>,
    pub associated_buckets: Vec<Bucket>,
    associated_buckets_set: FxHashSet<Bucket>,
}

impl RegEffectData {
    pub fn new() -> Self {
        RegEffectData {
            facets: Vec::new(),
            associated_buckets: Vec::new(),
            associated_buckets_set: FxHashSet::default(),
        }
    }

    pub fn add_facets(&mut self, facets: RegEffectFacets) {
        self.facets.push(facets);
    }

    pub fn update_buckets(&mut self, new_buckets: &FxHashSet<Bucket>) {
        for bucket in new_buckets - &self.associated_buckets_set {
            self.associated_buckets.push(bucket);
        }
        self.associated_buckets_set.extend(new_buckets.iter());
    }
}
