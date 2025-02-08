
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Addenda99Dishonored {
    pub original_entry_trace_number: String,
}

impl Addenda99Dishonored {
    pub fn original_entry_trace_number_field(&self) -> &str {
        &self.original_entry_trace_number
    }
}

impl FromStr for Addenda99Dishonored {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut original_entry_trace_number = String::new();
        let mut dishonored_return_reason_code = String::new();
        let mut original_receiving_dfi_identification = String::new();
        let mut return_trace_number = String::new();
        let mut return_settlement_date = String::new();
        let mut return_reason_code = String::new();
        let mut addenda_information = String::new();
        let mut trace_number = String::new();

        let mut lines = s.lines();
        while let Some(line) = lines.next() {
            let mut fields = line.splitn(2, ':');
            let field_name = fields.next().unwrap();
            let field_value = fields.next().unwrap();
            match field_name {
                "OriginalEntryTraceNumber" => original_entry_trace_number = field_value.to_string(),
                "DishonoredReturnReasonCode" => dishonored_return_reason_code = field_value.to_string(),
                "OriginalReceivingDFIIdentification" => original_receiving_dfi_identification = field_value.to_string(),
                "ReturnTraceNumber" => return_trace_number = field_value.to_string(),
                "ReturnSettlementDate" => return_settlement_date = field_value.to_string(),
                "ReturnReasonCode" => return_reason_code = field_value.to_string(),
                "AddendaInformation" => addenda_information = field_value.to_string(),
                "TraceNumber" => trace_number = field_value.to_string(),
                _ => {}
            }
        }

        Ok(Addenda99Dishonored {
            original_entry_trace_number,
        })
    }
}

impl Display for Addenda99Dishonored {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OriginalEntryTraceNumber: {}\n", self.original_entry_trace_number)
    }
}

fn main() {
    let s = "OriginalEntryTraceNumber: 1234567890\nDishonoredReturnReasonCode: R01\nOriginalReceivingDFIIdentification: 123456789\nReturnTraceNumber: 987654321\nReturnSettlementDate: 20230308\nReturnReasonCode: R02\nAddendaInformation: This is some additional information.\nTraceNumber: 1234567890";
    let addenda99_dishonored = Addenda99Dishonored::from_str(s).unwrap();
    println!("{}", addenda99_dishonored);
}
