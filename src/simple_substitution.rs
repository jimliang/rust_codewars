// http://www.codewars.com/kata/simple-substitution-cipher-helper/train/rust

struct Cipher {
    encoder: Vec<(char, char)>,
    decoder: Vec<(char,char)>,
}

impl Cipher {
    fn new(map1: &str, map2: &str) -> Cipher {
        Cipher {
            encoder: map1.chars().zip(map2.chars()).collect(),
            decoder: map2.chars().zip(map1.chars()).collect(),
        }
    }

    fn encode(&self, string: &str) -> String {
        self._turn(&self.encoder, string)
    }

    fn decode(&self, string: &str) -> String {
        self._turn(&self.decoder, string)
    }
    fn _turn(&self, e: &Vec<(char, char)>, string: &str) -> String {
        string.chars().map(|c| {
            e.iter().find(|x|x.0 == c).map_or(c, |y| y.1)
        }).collect()
    }
}


#[test]
fn examples() {
    let map1 = "abcdefghijklmnopqrstuvwxyz";
    let map2 = "etaoinshrdlucmfwypvbgkjqxz";

    let cipher = Cipher::new(map1, map2);

    assert_eq!(cipher.encode("abc"), "eta");
    assert_eq!(cipher.encode("xyz"), "qxz");
    assert_eq!(cipher.decode("eirfg"), "aeiou");
    assert_eq!(cipher.decode("erlang"), "aikcfu");
}