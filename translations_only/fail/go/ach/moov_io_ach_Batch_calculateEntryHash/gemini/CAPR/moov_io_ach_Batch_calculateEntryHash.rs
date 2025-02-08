
use std::convert::TryInto;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct MoovIoAchBatch {
    pub header: Box<MoovIoAchBatchHeader>,
    pub entries: Vec<MoovIoAchEntryDetail>,
    pub adv_entries: Vec<MoovIoAchAdvEntryDetail>,
}

impl MoovIoAchBatch {
    pub fn calculate_entry_hash(&self) -> i32 {
        let mut hash = 0;

        if !self.is_adv() {
            for entry in &self.entries {
                let entry_rdfi = entry.rdfi_identification.parse::<i32>().unwrap();
                hash += entry_rdfi;
            }
        } else {
            for entry in &self.adv_entries {
                let entry_rdfi = entry.rdfi_identification.parse::<i32>().unwrap();
                hash += entry_rdfi;
            }
        }

        hash.rem_euclid(10_i32.pow(10))
    }

    pub fn is_adv(&self) -> bool {
        self.header.standard_entry_class_code == moov_io_ach_ADV
    }

    pub fn get_header(&self) -> &MoovIoAchBatchHeader {
        &self.header
    }

    pub fn least_significant_digits(&self, v: i32, max_digits: u32) -> i32 {
        v % 10_i32.pow(max_digits as u32)
    }
}

#[derive(Debug, Clone)]
pub struct MoovIoAchBatchHeader {
    pub service_class_code: String,
    pub company_name: String,
    pub company_discretionary_data: String,
    pub company_identification: String,
    pub standard_entry_class_code: String,
    pub company_entry_description: String,
    pub company_descriptive_date: String,
    pub effective_entry_date: String,
    pub settlement_date: String,
    pub originator_status_code: String,
    pub odfi_identification: String,
    pub batch_number: String,
}

#[derive(Debug, Clone)]
pub struct MoovIoAchEntryDetail {
    pub rdfi_identification: String,
    pub dfi_account_number: String,
    pub amount: i32,
    pub identification_number: String,
    pub individual_name: String,
    pub discretionary_data: String,
    pub addenda_record_indicator: String,
    pub trace_number: String,
    pub addenda02: String,
    pub addenda05: String,
    pub addenda98: String,
    pub addenda99: String,
    pub addenda99_contested: String,
    pub addenda99_dishonored: String,
}

#[derive(Debug, Clone)]
pub struct MoovIoAchAdvEntryDetail {
    pub transaction_code: String,
    pub rdfi_identification: String,
    pub dfi_account_number: String,
    pub amount: i32,
    pub advice_routing_number: String,
    pub file_identification: String,
    pub ach_operator_data: String,
    pub individual_name: String,
    pub discretionary_data: String,
    pub addenda_record_indicator: String,
    pub ach_operator_routing_number: String,
    pub julian_day: String,
    pub sequence_number: String,
    pub addenda99: String,
}

const moov_io_ach_ADV: &str = "moov_io_ach_ADV";
const moov_io_ach_GLPrenoteDebit: i32 = 48;
const moov_io_ach_GLZeroDollarRemittanceDebit: i32 = 49;

fn moov_io_ach_aba8(rtn: &str) -> String {
    let n = rtn.chars().count();
    match n {
        10 => {
            if rtn.chars().next().unwrap() == '0' || rtn.chars().next().unwrap() == '1' {
                rtn[1..9].to_string()
            } else {
                "".to_string()
            }
        }
        8 | 9 => rtn[..8].to_string(),
        _ => "".to_string(),
    }
}
