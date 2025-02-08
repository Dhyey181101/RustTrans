
use std::collections::HashMap;
use std::fmt;

const ZERO: char = '0';

struct Addenda99Dishonored {
    trace_number: String,
    moov_io_ach_converters: Converters,
}

struct Converters;

impl Addenda99Dishonored {
    fn new(trace_number: String, moov_io_ach_converters: Converters) -> Self {
        Self {
            trace_number,
            moov_io_ach_converters,
        }
    }
}

struct EntryField {
    field_name: String,
    value: String,
}

impl EntryField {
    fn new(field_name: String, value: String) -> Self {
        Self { field_name, value }
    }
}

struct DetailRecordField {
    sequence_number: u8,
    entry_fields: Vec<EntryField>,
}

impl DetailRecordField {
    fn new(sequence_number: u8, entry_fields: Vec<EntryField>) -> Self {
        Self {
            sequence_number,
            entry_fields,
        }
    }
}

struct Record {
    record_type: char,
    detail_record_fields: Vec<DetailRecordField>,
}

impl Record {
    fn new(record_type: char, detail_record_fields: Vec<DetailRecordField>) -> Self {
        Self {
            record_type,
            detail_record_fields,
        }
    }
}

struct Format99 {
    records: Vec<Record>,
}

impl Format99 {
    fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    fn add_record(&mut self, record: Record) {
        self.records.push(record);
    }
}

impl fmt::Display for Format99 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for record in &self.records {
            writeln!(
                f,
                "{}: [",
                match record.record_type {
                    '1' => "Detail",
                    '2' => "Addenda",
                    _ => "Unknown",
                }
            )?;

            for detail_record_field in &record.detail_record_fields {
                writeln!(
                    f,
                    "\tSequence Number: {:<2},",
                    detail_record_field.sequence_number
                )?;

                for entry_field in &detail_record_field.entry_fields {
                    writeln!(
                        f,
                        "\t\tField Name: {:<10}, Value: {}",
                        entry_field.field_name, entry_field.value
                    )?;
                }
            }

            writeln!(f, "]")?;
        }

        Ok(())
    }
}
