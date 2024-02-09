use std::ops::Deref;
#[allow(unused)]
struct MP3{
    audio: Vec<u8>,
    artist:Option<String>,
    title:Option<String>,
}

impl Deref for MP3{
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target{
        &self.audio
    }
}


fn main() {

    let favoritesong = MP3{
        audio : vec![1, 2, 3],
        artist: Some(String::from("Nirvana")),
        title: Some(String::from("Smells Like Teen")),
    };

    println!("{:?}", *favoritesong);
    assert_eq!(vec![1, 2, 3], *favoritesong);

}