use crate::domain::{Address, Email, FullName, Hetu, Phone};

use super::typstworld::TypstWorld;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use typst::diag::SourceDiagnostic;
use typst::ecow::EcoVec;
use typst::foundations::{Dict, IntoValue};
use typst_layout::PagedDocument;
use typst_pdf::PdfOptions;

#[derive(Debug, Clone)]
pub struct Contract {
    pub(crate) full_name: FullName,
    pub(crate) address: Address,
    pub(crate) phone_number: Phone,
    pub(crate) email: Email,
    pub(crate) hetu: Hetu,
    pub(crate) start_time: DateTime<Utc>,
    pub(crate) used_outside_finland: bool,
}

impl From<Contract> for Dict {
    fn from(info: Contract) -> Self {
        let mut d = Self::new();
        d.insert("full_name".into(), info.full_name.as_str().into_value());
        d.insert("address".into(), info.address.as_str().into_value());
        d.insert("phone_number".into(), info.phone_number.to_string().into_value());
        d.insert("email".into(), info.email.as_str().into_value());
        d.insert("hetu".into(), info.hetu.as_str().into_value());
        d.insert("start_time".into(), info.start_time.to_string().into_value());
        d.insert(
            "used_outside_finland".into(),
            info.used_outside_finland.into_value(),
        );
        d
    }
}

pub fn generate_pdf(info: Contract) -> Result<Vec<u8>, PdfError> {
    let inputs = Dict::from(info);
    let world = TypstWorld::new(inputs);
    let doc: PagedDocument = typst::compile(&world).output.map_err(PdfError::Compile)?;

    typst_pdf::pdf(&doc, &PdfOptions::default()).map_err(PdfError::Export)
}

#[derive(Debug, thiserror::Error)]
pub enum PdfError {
    #[error("Typst compilation failed: {}", format_diagnostics(.0))]
    Compile(EcoVec<SourceDiagnostic>),

    #[error("pdf export failed: {}", format_diagnostics(.0))]
    Export(EcoVec<SourceDiagnostic>),
}

fn format_diagnostics(diags: &EcoVec<SourceDiagnostic>) -> String {
    diags
        .iter()
        .map(|d| d.message.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}
