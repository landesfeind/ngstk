
use sequence::Sequence;
use sequence::SequenceElement;
use sequence::aminoacid::Aminoacid;
use sequence::dna::DnaNucleotide;

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
    Complex,
}

/// Tries to identify the variation between the two sequences by stripping the 
/// common prefix and common suffix.
/// The returned tuple contains the length of the stripped prefix as well as the
/// parts that are substituted.
pub fn calculate_sequence_variation<E: SequenceElement, S1: Sequence<E>, S2: Sequence<E>>(
    s1: &S1,
    s2: &S2,
) -> Option<(usize, Vec<E>, Vec<E>)> {
    let mut norm_ref = s1.vec();
    let mut norm_alt = s2.vec();

    let max_prefix_length = if norm_ref.len() >= norm_alt.len() {
            norm_alt.len() 
        } else {
            norm_ref.len()
        };

    let prefix_length = (0..max_prefix_length)
        .take_while(|i| norm_ref[*i] == norm_alt[*i] )
        .count();

    norm_ref = norm_ref.into_iter().skip(prefix_length).collect();
    norm_alt = norm_alt.into_iter().skip(prefix_length).collect();

    // Reverse to get backward sequence
    norm_ref.reverse();
    norm_alt.reverse();

    let max_suffix_length = if norm_ref.len() >= norm_alt.len() {
            norm_alt.len() 
        } else {
            norm_ref.len()
        };

    let suffix_length = (0..max_suffix_length)
        .take_while(|i| norm_ref[*i] == norm_alt[*i] )
        .count();

    norm_ref = norm_ref.into_iter().skip(suffix_length).collect();
    norm_alt = norm_alt.into_iter().skip(suffix_length).collect();

    // Revese in-place back to original direction
    norm_ref.reverse();
    norm_alt.reverse();

    if norm_ref.len() == 0 && norm_alt.len() == 0 {
        None
    } else {
        Some((prefix_length, norm_ref, norm_alt))
    }
}



/// A variant is a variation of a sequence of `S`
/// by means of a change of a part of the sequence into
/// another.
pub trait Variant<E: SequenceElement> {
    type SequenceType: Sequence<E> + From<Vec<E>>;

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
        } else if rlen > 0 && rlen == alen {
            Some(VariantType::Substitution)
        } else if rlen == 0 && alen > 0 {
            Some(VariantType::Insertion)
        } else if rlen > 0 && alen == 0 {
            Some(VariantType::Deletion)
        } else {
            Some(VariantType::Complex)
        }
    }

    /// Normalizes the variant by removing a potential common sequence prefix.
    /// @returns a tuple containing the new offset, new normalized reference, and
    /// the normalized alternative.
    fn normalized_variation(&self) -> (usize, Self::SequenceType, Self::SequenceType) {
        match calculate_sequence_variation(&self.reference(), &self.alternative()) {
            None => {
                (
                    self.offset(),
                    Self::SequenceType::from(Vec::new()),
                    Self::SequenceType::from(Vec::new()),
                )
            }
            Some((offset, norm_ref, norm_alt)) => {
                (
                    self.offset() + offset,
                    Self::SequenceType::from(norm_ref),
                    Self::SequenceType::from(norm_alt),
                )
            }
        }
    }

    /// Applies the variant to the given reference sequence and returns the altered sequence.
    /// This method will not check if the reference matches.
    fn apply_variant<S: Sequence<E>>(&self, full_sequence: &S) -> Self::SequenceType {
        let (offset, reference, alternative) = self.normalized_variation();

        let mut new_seq: Vec<E> = full_sequence.subsequence(0, offset).vec();
        new_seq.append(&mut alternative.vec());
        new_seq.append(&mut full_sequence.subsequence(offset + reference.length(), full_sequence.length()).vec());

        Self::SequenceType::from(new_seq)
    }

    /// Compares the variants reference against the full template sequence.
    /// Of the variant's reference is located a the desinated offset in the full sequence,
    /// the function returns true.
    fn check_variant_reference<S: Sequence<E>>(&self, full_sequence: &S) -> bool {
        let s = full_sequence.subsequence(self.offset(), self.reference_length());
        s.vec() == self.reference().vec()
    }

    /// Describe the variant by standards of the Human Genome Variant Society
    /// @see www.hgvs.org
    fn hgvs(&self) -> String {
        let (offset, reference, alternative) = self.normalized_variation();
        if reference.length() == 0 {
            format!("{}_{}ins{}", offset + 1, offset + 2, alternative )
        } 
        else if alternative.length() == 0 && reference.length() == 1{
            format!("{}del{}", offset + 1, reference)
        }
        else if alternative.length() == 0 && reference.length() > 1{
            format!("{}_{}del{}", offset + 1, offset + reference.length(), reference)
        }
        else {
            format!("{}{}>{}", offset + 1, reference, alternative)
        }
    }
}


pub trait GenomicVariant: Variant<DnaNucleotide> {}

pub trait PeptideVariant: Variant<Aminoacid> {}


#[cfg(test)]
mod tests {
    use model::variant::*;
    use sequence::dna::*;

    struct MockVariant {
        offset: usize,
        refer: DnaSequence,
        alter: DnaSequence,
    }

    impl Variant<DnaNucleotide> for MockVariant {
        type SequenceType = DnaSequence;
        fn template(&self) -> String {
            "ref".to_string()
        }
        fn offset(&self) -> usize {
            self.offset
        }
        fn reference(&self) -> Self::SequenceType {
            self.refer.clone()
        }
        fn alternative(&self) -> Self::SequenceType {
            self.alter.clone()
        }
    }

    #[test]
    fn test_normalization_substition_normal() {
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
    fn test_normalization_substition_prefix() {
        let variant = MockVariant {
            offset: 10,
            refer: DnaSequence::from_str(&"ACGT").unwrap(),
            alter: DnaSequence::from_str(&"AGCG").unwrap(),
        };
        let normed = variant.normalized_variation();

        assert_eq!(normed.0, variant.offset() + 1);
        assert_eq!(normed.1, DnaSequence::from_str(&"CGT").unwrap());
        assert_eq!(normed.2, DnaSequence::from_str(&"GCG").unwrap());
    }

    #[test]
    fn test_normalization_substition_suffix() {
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
    fn test_normalization_substition_prefix_and_suffix() {
        let variant = MockVariant {
            offset: 10,
            refer: DnaSequence::from_str(&"ACGT").unwrap(),
            alter: DnaSequence::from_str(&"AGCT").unwrap(),
        };
        let normed = variant.normalized_variation();

        assert_eq!(normed.0, variant.offset() + 1);
        assert_eq!(normed.1, DnaSequence::from_str(&"CG").unwrap());
        assert_eq!(normed.2, DnaSequence::from_str(&"GC").unwrap());
    }

    #[test]
    fn test_check_reference() {
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

        assert!(variant_ok.check_variant_reference(&full_sequence));
        assert!(!variant_wrong.check_variant_reference(&full_sequence));
    }

    #[test]
    fn test_calculate_sequence_variation_none(){
        let seq1 = DnaSequence::from_str(&"ACGT").unwrap();
        assert_eq!( calculate_sequence_variation(&seq1.clone(), &seq1), None, "No variation found") ;
    }

    #[test]
    fn test_calculate_sequence_variation_substitution(){
        let seq1 = DnaSequence::from_str(&"ACGT").unwrap();
        let seq2 = DnaSequence::from_str(&"AGGT").unwrap();
        let varo = calculate_sequence_variation(&seq1, &seq2);
        assert!(varo.is_some());
        let var = varo.unwrap();

        assert_eq!(var.0, 1);
        assert_eq!(var.1, DnaSequence::from_str(&"C").unwrap().vec());
        assert_eq!(var.2, DnaSequence::from_str(&"G").unwrap().vec());
    }

    #[test]
    fn test_calculate_sequence_variation_insertion(){
        let seq1 = DnaSequence::from_str(&"ACGT").unwrap();
        let seq2 = DnaSequence::from_str(&"ACAGT").unwrap();
        let varo = calculate_sequence_variation(&seq1, &seq2);
        assert!(varo.is_some());
        let var = varo.unwrap();

        assert_eq!(var.0, 2);
        assert_eq!(var.1, DnaSequence::from_str(&"").unwrap().vec());
        assert_eq!(var.2, DnaSequence::from_str(&"A").unwrap().vec());
    }

    #[test]
    fn test_calculate_sequence_variation_deletion(){
        let seq1 = DnaSequence::from_str(&"ACGT").unwrap();
        let seq2 = DnaSequence::from_str(&"ACT").unwrap();
        let varo = calculate_sequence_variation(&seq1, &seq2);
        assert!(varo.is_some());
        let var = varo.unwrap();

        assert_eq!(var.0, 2);
        assert_eq!(var.1, DnaSequence::from_str(&"G").unwrap().vec());
        assert_eq!(var.2, DnaSequence::from_str(&"").unwrap().vec());
    }

}
