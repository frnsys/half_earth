use crate::kinds::FeedstockMap;

/// About a 27% increase per year
/// if expanded every month
const MONTHLY_EXPANSION_RATE: f32 = 0.02;

#[derive(Default)]
pub struct ExtractionManager {
    /// How much of each feedstock is extracted per month
    extraction_rates: FeedstockMap<f32>,

    /// The overall planetary limit for each feedstock
    reserves: FeedstockMap<f32>,
}

impl ExtractionManager {
    /// Extract feedstocks
    pub fn extract(&mut self) -> FeedstockMap<f32> {
        let mut extracted = feedstocks!();
        for (k, v) in self.extraction_rates.items() {
            let amount = f32::min(*v, self.reserves[k]);
            self.reserves[k] -= amount;
            extracted[k] += amount;
        }
        extracted
    }

    /// Try to expand/contract monthly
    /// extraction to meet monthly demand
    pub fn adjust(&mut self, demand: &FeedstockMap<f32>) {
        for (k, v) in demand.items() {
            let rate = &mut self.extraction_rates[k];
            let gap = *v - *rate;
            let change = if gap > 0. {
                f32::min(gap, *rate * MONTHLY_EXPANSION_RATE)
            } else {
                f32::max(gap, -*rate * MONTHLY_EXPANSION_RATE)
            };
            *rate += change;
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extraction() {
        let mut em = ExtractionManager {
            extraction_rates: feedstocks!(oil: 10., coal: 20.),
            reserves: feedstocks!(oil: 20., coal: 10.),
        };
        let extracted = em.extract();

        // Should not extract more than there is
        assert_eq!(extracted.coal, 10.);
        assert_eq!(extracted.oil, 10.);

        assert_eq!(em.reserves.oil, 10.);
        assert_eq!(em.reserves.coal, 0.);
    }


    #[test]
    fn test_adjust_extraction() {
        let mut em = ExtractionManager {
            extraction_rates: feedstocks!(oil: 1., coal: 2.),
            reserves: feedstocks!(oil: 20., coal: 10.),
        };
        let demand = feedstocks!(oil: 5., coal: 1.);
        em.adjust(&demand);

        // Expand
        assert_eq!(em.extraction_rates.oil, 1.02);

        // Contract
        assert_eq!(em.extraction_rates.coal, 1.96);
    }
}
