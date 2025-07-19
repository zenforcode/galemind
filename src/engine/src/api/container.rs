//! Data container abstractions for batching different types
//! 
//! This module provides traits and implementations for handling various data types
//! in batch operations, similar to BentoML's AutoContainer system.

use std::fmt::Debug;
use crate::error::{BatchError, Result};

/// Trait for types that can be batched together
pub trait Batchable: Send + Sync + Debug + Clone {
    /// Get the batch size of this item
    fn batch_size(&self) -> usize {
        1
    }
    
    /// Check if this item can be combined with others of the same type
    fn can_batch_with(&self, _other: &Self) -> bool {
        true
    }
}

/// Trait for combining multiple items into a batch
pub trait BatchCombiner<T: Batchable> {
    type Batch: Send + Sync + Debug;
    
    /// Combine individual items into a batch
    fn combine(items: Vec<T>) -> Result<Self::Batch>;
    
    /// Split a batch back into individual items
    fn split(batch: Self::Batch, sizes: &[usize]) -> Result<Vec<T>>;
    
    /// Get the total batch size
    fn batch_size(batch: &Self::Batch) -> usize;
}

/// Default implementation for Vec<T>
impl<T: Batchable> Batchable for Vec<T> {
    fn batch_size(&self) -> usize {
        self.iter().map(|item| item.batch_size()).sum()
    }
}

/// Batch combiner for vectors
pub struct VecBatchCombiner<T: Batchable> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Batchable> BatchCombiner<T> for VecBatchCombiner<T> {
    type Batch = Vec<T>;
    
    fn combine(items: Vec<T>) -> Result<Self::Batch> {
        Ok(items)
    }
    
    fn split(batch: Self::Batch, sizes: &[usize]) -> Result<Vec<T>> {
        if sizes.iter().sum::<usize>() != batch.len() {
            return Err(BatchError::InvalidBatchSize {
                size: batch.len(),
                max: sizes.iter().sum(),
            });
        }
        
        Ok(batch)
    }
    
    fn batch_size(batch: &Self::Batch) -> usize {
        batch.len()
    }
}

/// Batch combiner for nested vectors (Vec<Vec<T>>)
pub struct NestedVecBatchCombiner<T: Batchable> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Batchable> BatchCombiner<Vec<T>> for NestedVecBatchCombiner<T> {
    type Batch = Vec<T>;
    
    fn combine(items: Vec<Vec<T>>) -> Result<Self::Batch> {
        Ok(items.into_iter().flatten().collect())
    }
    
    fn split(batch: Self::Batch, sizes: &[usize]) -> Result<Vec<Vec<T>>> {
        let mut result = Vec::new();
        let mut offset = 0;
        
        for &size in sizes {
            if offset + size > batch.len() {
                return Err(BatchError::InvalidBatchSize {
                    size: batch.len(),
                    max: offset + size,
                });
            }
            
            result.push(batch[offset..offset + size].to_vec());
            offset += size;
        }
        
        Ok(result)
    }
    
    fn batch_size(batch: &Self::Batch) -> usize {
        batch.len()
    }
}

// Implement Batchable for common types
impl Batchable for i32 {}
impl Batchable for i64 {}
impl Batchable for f32 {}
impl Batchable for f64 {}
impl Batchable for String {}
impl Batchable for &str {}

// Optional ndarray support
#[cfg(feature = "ndarray-support")]
mod ndarray_support {
    use super::*;
    use ndarray::{Array, Dimension};
    
    impl<T, D> Batchable for Array<T, D>
    where
        T: Clone + Send + Sync + Debug,
        D: Dimension + Send + Sync + Debug,
    {
        fn batch_size(&self) -> usize {
            if self.ndim() == 0 {
                1
            } else {
                self.shape()[0]
            }
        }
    }
}

// Optional nalgebra support
#[cfg(feature = "nalgebra-support")]
mod nalgebra_support {
    use super::*;
    use nalgebra::{DMatrix, DVector};
    
    impl<T> Batchable for DMatrix<T>
    where
        T: Clone + Send + Sync + Debug + nalgebra::Scalar,
    {
        fn batch_size(&self) -> usize {
            self.nrows()
        }
    }
    
    impl<T> Batchable for DVector<T>
    where
        T: Clone + Send + Sync + Debug + nalgebra::Scalar,
    {
        fn batch_size(&self) -> usize {
            self.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec_batch_combiner() {
        let items = vec![1, 2, 3, 4, 5];
        let batch = VecBatchCombiner::combine(items.clone()).unwrap();
        assert_eq!(VecBatchCombiner::batch_size(&batch), 5);
        
        let split_result = VecBatchCombiner::split(batch, &[5]).unwrap();
        assert_eq!(split_result, items);
    }
    
    #[test]
    fn test_nested_vec_batch_combiner() {
        let items = vec![vec![1, 2], vec![3, 4, 5]];
        let batch = NestedVecBatchCombiner::combine(items).unwrap();
        assert_eq!(NestedVecBatchCombiner::batch_size(&batch), 5);
        
        let split_result = NestedVecBatchCombiner::split(batch, &[2, 3]).unwrap();
        assert_eq!(split_result, vec![vec![1, 2], vec![3, 4, 5]]);
    }
}
