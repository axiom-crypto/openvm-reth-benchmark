use revm_primitives::{b256, keccak256};

use crate::{Error, MptTrie};

trait RlpBytes {
    /// Returns the RLP-encoding.
    fn to_rlp(&self) -> Vec<u8>;
}

impl<T> RlpBytes for T
where
    T: alloy_rlp::Encodable,
{
    #[inline]
    fn to_rlp(&self) -> Vec<u8> {
        let rlp_length = self.length();
        let mut out = Vec::with_capacity(rlp_length);
        self.encode(&mut out);
        debug_assert_eq!(out.len(), rlp_length);
        out
    }
}

#[test]
fn test_empty() {
    let bump = bumpalo::Bump::new();
    let trie = MptTrie::new(&bump);

    assert!(trie.is_empty());
    let expected = b256!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421");
    assert_eq!(expected, trie.hash());
}

#[test]
fn test_empty_key() -> Result<(), Error> {
    let bump = bumpalo::Bump::new();
    let mut trie = MptTrie::new(&bump);

    trie.insert(&[], b"empty")?;
    assert_eq!(trie.get(&[])?, Some(b"empty".as_ref()));
    assert!(trie.delete(&[])?);

    Ok(())
}

#[test]
fn test_branch_value() {
    let bump = bumpalo::Bump::new();
    let mut trie = MptTrie::new(&bump);
    trie.insert(b"do", b"verb").unwrap();
    // leads to a branch with value which is not supported
    trie.insert(b"dog", b"puppy").unwrap_err();
}

#[test]
fn test_insert() -> Result<(), Error> {
    let bump = bumpalo::Bump::new();
    let mut trie = MptTrie::new(&bump);

    let key_vals = vec![
        ("painting", "place"),
        ("guest", "ship"),
        ("mud", "leave"),
        ("paper", "call"),
        ("gate", "boast"),
        ("tongue", "gain"),
        ("baseball", "wait"),
        ("tale", "lie"),
        ("mood", "cope"),
        ("menu", "fear"),
    ];
    for (key, val) in &key_vals {
        assert!(trie.insert(key.as_bytes(), val.as_bytes())?);
    }

    let expected = b256!("2bab6cdf91a23ebf3af683728ea02403a98346f99ed668eec572d55c70a4b08f");
    assert_eq!(expected, trie.hash());

    for (key, value) in &key_vals {
        let retrieved = trie.get(key.as_bytes())?.unwrap();
        assert_eq!(retrieved, value.as_bytes());
    }

    // check inserting duplicate keys
    assert!(trie.insert(key_vals[0].0.as_bytes(), b"new")?);
    assert!(!trie.insert(key_vals[0].0.as_bytes(), b"new")?);

    Ok(())
}

#[test]
fn test_keccak_trie() -> Result<(), Error> {
    const N: usize = 512;

    // insert
    let bump = bumpalo::Bump::new();
    let mut trie = MptTrie::new(&bump);

    for i in 0..N {
        assert!(trie.insert_rlp(keccak256(i.to_be_bytes()).as_slice(), i)?);

        // check hash against trie build in reverse
        let bump2 = bumpalo::Bump::new();
        let mut trie2 = MptTrie::new(&bump2);
        for j in (0..=i).rev() {
            trie2.insert_rlp(keccak256(j.to_be_bytes()).as_slice(), j)?;
        }
        assert_eq!(trie.hash(), trie2.hash());
    }

    let expected = b256!("7310027edebdd1f7c950a7fb3413d551e85dff150d45aca4198c2f6315f9b4a7");
    assert_eq!(trie.hash(), expected);

    // get
    for i in 0..N {
        assert_eq!(trie.get_rlp(keccak256(i.to_be_bytes()).as_slice())?, Some(i));
        assert!(trie.get(keccak256((i + N).to_be_bytes()).as_slice())?.is_none());
    }

    // delete
    for i in 0..N {
        assert!(trie.delete(keccak256(i.to_be_bytes()).as_slice())?);

        let bump2 = bumpalo::Bump::new();
        let mut trie2 = MptTrie::new(&bump2);
        for j in ((i + 1)..N).rev() {
            trie2.insert_rlp(keccak256(j.to_be_bytes()).as_slice(), j)?;
        }
        assert_eq!(trie.hash(), trie2.hash());
    }
    assert!(trie.is_empty());

    Ok(())
}

#[test]
fn test_index_trie() -> Result<(), Error> {
    const N: usize = 512;

    // insert
    let bump = bumpalo::Bump::new();
    let mut trie = MptTrie::new(&bump);

    for i in 0..N {
        assert!(trie.insert_rlp(&i.to_rlp(), i)?);

        // check hash against trie build in reverse
        let bump2 = bumpalo::Bump::new();
        let mut trie2 = MptTrie::new(&bump2);
        for j in (0..=i).rev() {
            trie2.insert_rlp(&j.to_rlp(), j)?;
        }
        assert_eq!(trie.hash(), trie2.hash());
    }

    // get
    for i in 0..N {
        assert_eq!(trie.get_rlp(&i.to_rlp())?, Some(i));
        assert!(trie.get(&(i + N).to_rlp())?.is_none());
    }

    // delete
    for i in 0..N {
        assert!(trie.delete(&i.to_rlp()).unwrap());

        let bump2 = bumpalo::Bump::new();
        let mut trie2 = MptTrie::new(&bump2);
        for j in ((i + 1)..N).rev() {
            trie2.insert_rlp(&j.to_rlp(), j)?;
        }
        assert_eq!(trie.hash(), trie2.hash());
    }
    assert!(trie.is_empty());

    Ok(())
}

#[test]
fn test_serde_index_trie() -> Result<(), Error> {
    const N: usize = 512;

    let bump = bumpalo::Bump::new();
    let mut trie = MptTrie::new(&bump);

    for i in 0..N {
        assert!(trie.insert_rlp(&i.to_rlp(), i)?);
    }
    // trie.print_tree();

    let root_hash = trie.hash();

    let encoded = trie.encode_trie();
    // println!("{encoded:?}");

    let recovered_trie = MptTrie::decode_trie(&bump, &mut encoded.as_slice(), trie.num_nodes())?;
    // recovered_trie.print_tree();
    assert_eq!(recovered_trie.hash(), root_hash);

    for i in 0..N {
        let value = recovered_trie.get_rlp(&i.to_rlp())?;
        assert_eq!(value, Some(i));
    }

    Ok(())
}

#[test]
fn test_serde_keccak_trie() -> Result<(), Error> {
    const N: usize = 512;

    let bump = bumpalo::Bump::new();
    let mut trie = MptTrie::new(&bump);

    for i in 0..N {
        assert!(trie.insert_rlp(keccak256(i.to_be_bytes()).as_slice(), i)?);
    }
    // trie.print_tree();

    let root_hash = trie.hash();

    let encoded = trie.encode_trie();

    let recovered_trie = MptTrie::decode_trie(&bump, &mut encoded.as_slice(), trie.num_nodes())?;
    assert_eq!(recovered_trie.hash(), root_hash);

    for i in 0..N {
        let value = recovered_trie.get_rlp(keccak256(i.to_be_bytes()).as_slice())?;
        assert_eq!(value, Some(i));
    }

    Ok(())
}
