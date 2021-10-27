#[derive(thiserror::Error, Debug, PartialEq)]
pub enum BioError {
	#[error("Expected nucleotide, found {0}")]
	NotNucleotide(u8),
}

pub type BioResult<T> = Result<T, BioError>;
