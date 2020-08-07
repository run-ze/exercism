pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => format!("{num} bottle of beer on the wall, {num} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n",num=n),
        2 => format!("{num} bottles of beer on the wall, {num} bottles of beer.\nTake one down and pass it around, {last_num} bottle of beer on the wall.\n",num=n,last_num=n-1),
        _ => format!("{num} bottles of beer on the wall, {num} bottles of beer.\nTake one down and pass it around, {last_num} bottles of beer on the wall.\n",num=n,last_num=n-1),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = verse(start);
    for i in (end..start).rev() {
        song.push_str("\n");
        song.push_str(&verse(i));
    }
    song
}
