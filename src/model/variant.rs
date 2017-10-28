use sequence::SequenceElement;
use sequence::Sequence;
use sequence::dna::DnaNucleotide;
use sequence::aminoacid::Aminoacid;

pub enum VariantType {
	/// A pseudo variation that does not change the original sequence.
	None, 
	/// A variation of the sequence by exchaning a part of the sequence 
	/// with an equally long alternative.
	Substitution, 
	/// A variation by which an additional sequence is inserted into the
	/// original sequence.
	Insertion, 
	/// A variation by which a part of the original sequence is removed
	/// without replacement.
	Deletion, 
	/// A variation that is none of the above.
	Complex
}


/// A variant is a variation of a sequence of `S` 
/// by means of a change of a part of the sequence into
/// another.
pub trait Variant<E: SequenceElement> {
	type SequenceType : Sequence<E>;

	/// Returns the name of the template (e.g., the chromosome) 
	/// on which this variant is located.
	fn template(&self) -> String;

	/// Returns the number of sequence elements that come
	/// before the start of the variant.
	fn offset(&self) -> usize;

	/// Returns the variant reference. That is the original 
	/// sequence that starts after position `offset()`.
	fn reference(&self) -> Self::SequenceType;

	/// Returns the length of the original sequence
	/// that is modified by this variant.
	fn reference_length(&self) -> usize {
		self.reference().length()
	}

	/// Returns the variant alternative. That is the new 
	/// sequence that starts after position `offset()`.
	fn alternative(&self) -> Self::SequenceType;

	/// Returns the length of the changed sequence
	/// that is modified by this variant.
	fn alternative_length(&self) -> usize {
		self.reference().length()
	}

	/// Returns the type of the variant.
	fn variant_type(&self) -> Option<VariantType> {
		let normed = self.normalized_variation();
		let rlen = normed.1.length();
		let alen = normed.2.length();

		if rlen == 0 && alen == 0 {
			None
		}
		else if rlen > 0 && rlen == alen {
			Some(VariantType::Substitution)
		} 
		else if rlen == 0 && alen > 0 {
			Some(VariantType::Insertion)
		}
		else if rlen > 0 && alen == 0{
			Some(VariantType::Deletion)
		}
		else {
			Some(VariantType::Complex)
		}
	}

	/// Normalizes the variant by removing a potential common sequence prefix.
	/// @returns a tuple containing the new offset, new normalized reference, and
	/// the normalized alternative.
	fn normalized_variation(&self) -> (usize, Self::SequenceType, Self::SequenceType) {
		let mut norm_ref = self.reference().as_vec();
		let mut norm_alt = self.alternative().as_vec();

		let prefix_length = norm_ref.iter().enumerate()
				.take_while(|es| norm_alt[es.0] == *(es.1))
				.count()
			;

		norm_ref = norm_ref.into_iter().skip(prefix_length).collect();
		norm_alt = norm_alt.into_iter().skip(prefix_length).collect();

		// Reverse to get backward sequence
		norm_ref.reverse();
		norm_alt.reverse();
		// find length of common prefix
		let suffix_length = norm_ref.iter().enumerate()
				.take_while(|es| norm_alt[es.0] == *(es.1))
				.count()
			;
		norm_ref = norm_ref.into_iter().skip(suffix_length).collect();
		norm_alt = norm_alt.into_iter().skip(suffix_length).collect();

		// Revese in-place back to original direction
		norm_ref.reverse();
		norm_alt.reverse();

		(self.offset() + prefix_length, Self::SequenceType::from(norm_ref), Self::SequenceType::from(norm_alt))
	}

	fn apply_variant<S: Sequence<E>>(&self, seq: &S) -> Self::SequenceType {
		let (offset, reference, alternative) = self.normalized_variation();

		let mut new_seq : Vec<E> = seq.iterator().take(offset).map(|e| e.clone() ).collect();
		
		let old_ref_from_seq : Vec<E> = seq.iterator().skip(offset).take(reference.length()).map(|e| e.clone() ).collect();
		if old_ref_from_seq != reference.as_vec() {
			let var_ref_string : String = reference.iterator().map(|e| e.to_string()).collect();
			let seq_ref_string : String = old_ref_from_seq.iter().map(|e| e.to_string()).collect();
			warn!("Apply variant to sequence but have mismatch in reference sequence (offset: {}). Expected '{:?}' from variant, found '{:?}' from input sequence.", offset, var_ref_string, seq_ref_string);
		}

		new_seq.append(&mut alternative.as_vec());
		new_seq.append(&mut seq.iterator().skip(offset + reference.length()).map(|e| e.clone() ).collect());

		Self::SequenceType::from(new_seq)
	}

	fn check_variant_reference<S: Sequence<E>>(&self, full_sequence: &S) -> bool {
		let s = full_sequence.subsequence(self.offset(), self.reference_length());
		s.as_vec() == self.reference().as_vec()
	}
}

pub trait GenomicVariant : Variant<DnaNucleotide> {}
pub trait PeptideVariant : Variant<Aminoacid> {}


#[cfg(test)]
mod tests {
	use model::variant::*;
	use sequence::*;
	use sequence::dna::*;

	struct MockVariant {
		offset: usize,
		refer: DnaSequence,
		alter: DnaSequence
	}

	impl Variant<DnaNucleotide> for MockVariant {
		type SequenceType = DnaSequence;
		fn template(&self) -> String { "ref".to_string() }
		fn offset(&self) -> usize { self.offset }
		fn reference(&self) -> Self::SequenceType { self.refer.clone() }
		fn alternative(&self) -> Self::SequenceType { self.alter.clone() }
	}

	#[test]
	fn test_normalization_substition_normal(){
		let variant = MockVariant {
			offset: 10,
			refer: DnaSequence::from_str(&"ACGT").unwrap(),
			alter: DnaSequence::from_str(&"CGCG").unwrap(),
		};
		let normed = variant.normalized_variation();

		assert_eq!(normed.0, variant.offset());
		assert_eq!(normed.1, DnaSequence::from_str(&"ACGT").unwrap());
		assert_eq!(normed.2, DnaSequence::from_str(&"CGCG").unwrap());
	}

	#[test]
	fn test_normalization_substition_prefix(){
		let variant = MockVariant {
			offset: 10,
			refer: DnaSequence::from_str(&"ACGT").unwrap(),
			alter: DnaSequence::from_str(&"AGCG").unwrap(),
		};
		let normed = variant.normalized_variation();

		assert_eq!(normed.0, variant.offset()+1);
		assert_eq!(normed.1, DnaSequence::from_str(&"CGT").unwrap());
		assert_eq!(normed.2, DnaSequence::from_str(&"GCG").unwrap());
	}

	#[test]
	fn test_normalization_substition_suffix(){
		let variant = MockVariant {
			offset: 10,
			refer: DnaSequence::from_str(&"ACGT").unwrap(),
			alter: DnaSequence::from_str(&"CGCT").unwrap(),
		};
		let normed = variant.normalized_variation();

		assert_eq!(normed.0, variant.offset());
		assert_eq!(normed.1, DnaSequence::from_str(&"ACG").unwrap());
		assert_eq!(normed.2, DnaSequence::from_str(&"CGC").unwrap());
	}

	#[test]
	fn test_normalization_substition_prefix_and_suffix(){
		let variant = MockVariant {
			offset: 10,
			refer: DnaSequence::from_str(&"ACGT").unwrap(),
			alter: DnaSequence::from_str(&"AGCT").unwrap(),
		};
		let normed = variant.normalized_variation();

		assert_eq!(normed.0, variant.offset()+1);
		assert_eq!(normed.1, DnaSequence::from_str(&"CG").unwrap());
		assert_eq!(normed.2, DnaSequence::from_str(&"GC").unwrap());
	}



	#[test]
	fn test_check_reference(){
		let full_sequence = DnaSequence::from_str(&"TACGTAGT").unwrap();
		let variant_ok = MockVariant {
			offset: 1,
			refer: DnaSequence::from_str(&"ACGT").unwrap(),
			alter: DnaSequence::from_str(&"AGCT").unwrap(),
		};
		let variant_wrong = MockVariant {
			offset: 1,
			refer: DnaSequence::from_str(&"CCGT").unwrap(),
			alter: DnaSequence::from_str(&"AGCT").unwrap(),
		};

		assert!(  variant_ok.check_variant_reference(&full_sequence));
		assert!(! variant_wrong.check_variant_reference(&full_sequence));
	}


}
