#[derive(thiserror::Error, Debug)]
pub enum BioError {
	#[error("Expected nucleotide, found {0}")]
	NotNucleotide(u8),
	#[error("Cannot open file")]
	FileError(#[from] std::io::Error),
	#[error("Cannot convert {0}")]
	TryFromInt(#[from] std::num::TryFromIntError),
	#[error("Cannot parse {0}")]
	ParseIntError(#[from] std::num::ParseIntError),
	#[error("Cannot convert BigUint to f64")]
	BigUintToF64,
}

pub type BioResult<T> = Result<T, BioError>;
