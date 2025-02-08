

use std::fmt;
use std::collections::HashMap;

struct MoovIoAchConverters;

impl MoovIoAchConverters {
fn numeric_field(&self, n: i32, max: u32) -> String {
let s = n.to_string();
if s.len() as u32 > max {
return s[(s.len() as usize - max as usize)..].to_string();
} else {
let m = max as i32 - s.len() as i32;
return "n".repeat(m as usize) + &s;
}
}
}

struct Record {
field1: i32,
field2: i32,
field3: i32,
field4: i32,
field5: i32,
field6: i32,
field7: i32,
field8: i32,
converters: Box<MoovIoAchConverters>,
}

impl Record {
fn new(
field1: i32,
field2: i32,
field3: i32,
field4: i32,
field5: i32,
field6: i32,
field7: i32,
field8: i32,
) -> Record {
Record {
field1,
field2,
field3,
field4,
field5,
field6,
field7,
field8,
converters: Box::new(MoovIoAchConverters {}),
}
}
}

struct RecordBatch {
records: Vec<Record>,
}

impl RecordBatch {
fn new(records: Vec<Record>) -> RecordBatch {
RecordBatch { records }
}
}

struct File {
record_batches: Vec<RecordBatch>,
converters: Box<MoovIoAchConverters>,
}

impl File {
fn new(record_batches: Vec<RecordBatch>) -> File {
File {
record_batches: record_batches,
converters: Box::new(MoovIoAchConverters {}),
}
}
}

struct Field {
name: String,
value: String,
}

impl Field {
fn new(name: &str, value: &str) -> Field {
Field {
name: name.to_string(),
value: value.to_string(),
}
}
}

struct Line {
fields: Vec<Field>,
}

impl Line {
fn new(fields: Vec<Field>) -> Line {
Line { fields }
}
}

struct Lines {
lines: Vec<Line>,
}

impl Lines {
fn new(lines: Vec<Line>) -> Lines {
Lines { lines }
}
}

impl fmt::Display for MoovIoAchConverters {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
write!(f, "")
}
}

impl fmt::Display for Record {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
let field1_display = if self.field1 < 0 {
"-".to_string() + &self.converters.numeric_field(self.field1.abs(), 7)
} else {
self.converters.numeric_field(self.field1, 7)
};
let field2_display = if self.field2 < 0 {
"-".to_string() + &self.converters.numeric_field(self.field2.abs(), 11)
} else {
self.converters.numeric_field(self.field2, 11)
};
let field3_display = if self.field3 < 0 {
"-".to_string() + &self.converters.numeric_field(self.field3.abs(), 7)
} else {
self.converters.numeric_field(self.field3, 7)
};
let field4_display = if self.field4 < 0 {
"-".to_string() + &self.converters.numeric_field(self.field4.abs(), 15)
} else {
self.converters.numeric_field(self.field4, 15)
};
let field5_display = if self.field5 < 0 {
"-".to_string() + &self.converters.numeric_field(self.field5.abs(), 11)
} else {
self.converters.numeric_field(self.field5, 11)
};
let field6_display = if self.field6 < 0 {
"-".to_string() + &self.converters.numeric_field(self.field6.abs(), 11)
} else {
self.converters.numeric_field(self.field6, 11)
};
let field7_display = if self.field7 < 0 {
"-".to_string() + &self.converters.numeric_field(self.field7.abs(), 11)
} else {
self.converters.numeric_field(self.field7, 11)
};
let field8_display = if self.field8 < 0 {
"-".to_string() + &self.converters.numeric_field(self.field8.abs(), 11)
} else {
self.converters.numeric_field(self.field8, 11)
};
writeln!(
f,
"{:<7} {:<11} {:<7} {:<15} {:<11} {:<11} {:<11} {:<11}",
field1_display,
field2_display,
field3_display,
field4_display,
field5_display,
field6_display,
field7_display,
field8_display
)
}
}

impl fmt::Display for RecordBatch {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
for record in &self.records {
writeln!(f, "{}", record)?;
}
Ok(())
}
}

impl fmt::Display for File {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
for record_batch in &self.record_batches {
writeln!(f, "{}", record_batch)?;
}
Ok(())
}
}

impl fmt::Display for Line {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
for field in &self.fields {
writeln!(f, "{}: {}", field.name, field.value)?;
}
Ok(())
}
}

impl fmt::Display for Lines {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
for line in &self.lines {
writeln!(f, "{}", line)?;
}
Ok(())
}
}

