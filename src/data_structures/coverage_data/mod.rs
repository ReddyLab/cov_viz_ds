use std::fmt;

use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

use crate::data_structures::{ChromosomeData, Facet};

pub mod serialize;

#[derive(Debug)]
pub struct CoverageData {
    pub chromosomes: Vec<ChromosomeData>,
    pub facets: Vec<Facet>,
    pub chrom_lengths: Vec<usize>,
}

const COVERAGE_DATA_FIELD_CHROMOSOMES: &str = "chromosomes";
const COVERAGE_DATA_FIELD_FACETS: &str = "facets";
const COVERAGE_DATA_FIELD_CHROM_LENGTHS: &str = "chrom_lengths";

impl CoverageData {
    pub fn new(
        chromsomes: Vec<ChromosomeData>,
        facets: Vec<Facet>,
        chrom_lengths: Vec<usize>,
    ) -> Self {
        CoverageData {
            chromosomes: chromsomes,
            facets: facets,
            chrom_lengths: chrom_lengths,
        }
    }

    // Uncomment if needed. This can be useful for debugging
    // pub fn print_chroms(&self) {
    //     for chrom in &self.chromosomes {
    //         chrom.print_info();
    //     }
    // }
}

impl Serialize for CoverageData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CoverageData", 2)?;
        state.serialize_field(COVERAGE_DATA_FIELD_CHROMOSOMES, &self.chromosomes)?;
        state.serialize_field(COVERAGE_DATA_FIELD_FACETS, &self.facets)?;
        state.serialize_field(COVERAGE_DATA_FIELD_CHROM_LENGTHS, &self.chrom_lengths)?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for CoverageData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Chromosomes,
            Facets,
            Chrom_Lengths,
        }

        struct CoverageDataVisitor;

        impl<'de> Visitor<'de> for CoverageDataVisitor {
            type Value = CoverageData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct CoverageData")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<CoverageData, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let chromosomes = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let facets = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let chrom_lengths = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(CoverageData::new(chromosomes, facets, chrom_lengths))
            }

            fn visit_map<V>(self, mut map: V) -> Result<CoverageData, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut chromosomes = None;
                let mut facets = None;
                let mut chrom_lengths = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Chromosomes => {
                            if chromosomes.is_some() {
                                return Err(de::Error::duplicate_field(
                                    COVERAGE_DATA_FIELD_CHROMOSOMES,
                                ));
                            }
                            chromosomes = Some(map.next_value()?);
                        }
                        Field::Facets => {
                            if facets.is_some() {
                                return Err(de::Error::duplicate_field(COVERAGE_DATA_FIELD_FACETS));
                            }
                            facets = Some(map.next_value()?);
                        }
                        Field::Chrom_Lengths => {
                            if chrom_lengths.is_some() {
                                return Err(de::Error::duplicate_field(COVERAGE_DATA_FIELD_FACETS));
                            }
                            chrom_lengths = Some(map.next_value()?);
                        }
                    }
                }
                let chromosomes = chromosomes
                    .ok_or_else(|| de::Error::missing_field(COVERAGE_DATA_FIELD_CHROMOSOMES))?;
                let facets =
                    facets.ok_or_else(|| de::Error::missing_field(COVERAGE_DATA_FIELD_FACETS))?;
                let chrom_lengths = chrom_lengths
                    .ok_or_else(|| de::Error::missing_field(COVERAGE_DATA_FIELD_CHROM_LENGTHS))?;
                Ok(CoverageData::new(chromosomes, facets, chrom_lengths))
            }
        }

        const FIELDS: &'static [&'static str] = &[
            COVERAGE_DATA_FIELD_CHROMOSOMES,
            COVERAGE_DATA_FIELD_FACETS,
            COVERAGE_DATA_FIELD_CHROM_LENGTHS,
        ];
        deserializer.deserialize_struct("CoverageData", FIELDS, CoverageDataVisitor)
    }
}
