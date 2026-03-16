use std::io::Cursor;

use arrow_array::RecordBatch;
use arrow_ipc::{reader::StreamReader, writer::StreamWriter};

use crate::{Result, error::Error};

pub fn encode_record_batches_ipc(batches: &[RecordBatch]) -> Result<Vec<u8>> {
    let schema = batches.first().ok_or(Error::EmptyRecordBatches)?.schema();
    let mut payload = Vec::new();
    let mut writer = StreamWriter::try_new(&mut payload, &schema)?;

    for batch in batches {
        writer.write(batch)?;
    }

    writer.finish()?;

    Ok(payload)
}

pub fn decode_record_batches_ipc(payload: impl AsRef<[u8]>) -> Result<Vec<RecordBatch>> {
    let reader = StreamReader::try_new(Cursor::new(payload.as_ref()), None)?;
    let mut batches = Vec::new();

    for batch in reader {
        batches.push(batch?);
    }

    Ok(batches)
}
