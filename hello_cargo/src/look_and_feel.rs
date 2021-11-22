fn main () {
    //行末の改行をエスケープで消す
    let penguin_data = "\
    common name/length (cm)
    Little penguin, 33
    Yellow-eyed penguin, 65
    Fiordland penguin, 60
    Invalid, data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0{
            continue;
        }
    

        //Vec<_>は型注釈 vecはvectorというコレクション型の名称で、_は要素の型をrustに推測させる
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        //printlnは標準出力だけど、eprintlnはエラー出力部分
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields)
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>(){
            println!{"{}, {}cm", name, length};
        }
    }   
}