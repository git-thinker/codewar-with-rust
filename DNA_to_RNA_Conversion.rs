// https://www.codewars.com/kata/5556282156230d0e5e000089

fn dna_to_rna(dna: &str) -> String {
    let rna = dna.to_string();
    let rna:Vec<_> = rna.split('T').collect();
    let rna = rna.join(&"U");
    rna
}

#[cfg(test)]
mod tests {
    use super::dna_to_rna;

    #[test]
    fn returns_expected() {
      assert_eq!(dna_to_rna("TTTT"), "UUUU");
      assert_eq!(dna_to_rna("GCAT"), "GCAU");
    }
}