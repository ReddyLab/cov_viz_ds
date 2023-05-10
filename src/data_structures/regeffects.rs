use serde::{Deserialize, Serialize};

use crate::data_structures::DbID;
use rustc_hash::FxHashSet;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct BucketLoc {
    pub chrom: u8, // Chromosome index, e.g. chr1 has index 0, chrX is index 22
    pub idx: u32,  // intra-chromosome bucket index
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ObservationData {
    pub reo_id: DbID,
    // facet_ids should contain the IDs of both the REO's and the associated feature's
    // discrete facet values
    pub facet_ids: Vec<DbID>,
    pub effect_size: f32,
    pub significance: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FeatureData {
    pub id: DbID,
    pub facets: Vec<ObservationData>,
    pub associated_buckets: Vec<BucketLoc>,
    associated_buckets_set: FxHashSet<BucketLoc>,
}

impl FeatureData {
    pub fn new(id: DbID) -> Self {
        FeatureData {
            id,
            facets: Vec::new(),
            associated_buckets: Vec::new(),
            associated_buckets_set: FxHashSet::default(),
        }
    }

    pub fn add_facets(&mut self, facets: ObservationData) {
        self.facets.push(facets);
    }

    pub fn update_buckets(&mut self, new_buckets: &FxHashSet<BucketLoc>) {
        for bucket in new_buckets - &self.associated_buckets_set {
            self.associated_buckets.push(bucket);
        }
        self.associated_buckets_set.extend(new_buckets.iter());
    }
}
