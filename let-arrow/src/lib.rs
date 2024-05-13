use arrow::array::{ArrayRef, Int32Array, Int64Array, PrimitiveArray};
use arrow::datatypes::{ArrowPrimitiveType, Int32Type, Int64Type};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

pub trait RandomPrimitiveArray {
    type Item;
    fn get_value() -> PrimitiveArray<Self::Item>
    where
        Self::Item: ArrowPrimitiveType;
}

impl RandomPrimitiveArray for Int64Array {
    type Item = Int64Type;
    fn get_value() -> PrimitiveArray<Self::Item> {
        Int64Array::from_iter_values(vec![1, 2, 3])
    }
}
impl RandomPrimitiveArray for Int32Array {
    type Item = Int32Type;
    fn get_value() -> PrimitiveArray<Self::Item> {
        Int32Array::from_iter_values(vec![1, 2, 3])
    }
}

pub fn rb() -> RecordBatch {
    let col1 = Arc::new(Int64Array::from_iter_values([1, 2, 3])) as ArrayRef;
    let col2 = Arc::new(Int32Array::from_iter_values([1, 2, 3])) as ArrayRef;
    let to_write = RecordBatch::try_from_iter([("col1", col1), ("col2", col2)]).unwrap();
    to_write
}

// pub async fn arrow_writer() {
//     use parquet::arrow::AsyncArrowWriter;
//     let mut file = tokio::fs::File::options().create(true).write(true).open("tem.csv").await.unwrap();
//     let rb = rb();
//     let mut writer = AsyncArrowWriter::try_new(&mut file, rb.schema(), None).unwrap();
//     writer.write(&rb).await.expect("not able to write");
//     writer.close().await.expect("something went wrong");
// }
