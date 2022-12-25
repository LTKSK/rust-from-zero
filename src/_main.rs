use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock},
    thread::sleep,
    time::Duration,
};

/// main is a function
fn main() {
    let mut gallery = BTreeMap::new();
    gallery.insert("葛飾北斎", "富嶽三十六景  神奈川沖浪裏");
    gallery.insert("ミュシャ", "黄道十二宮");

    let gallery = Arc::new(RwLock::new(gallery));

    let mut hdls = Vec::new();//joinはんどら
    for n in 0..3 {
        let gallery = gallery.clone();
        let hdl = std::thread::spawn(move || {
            for _ in 0..8 {
                {
                    let guard = gallery.read().unwrap(); // read lock
                    if n == 0 {
                        for (key, value) in guard.iter() {
                            print!("{key}:{value}, ");
                        }
                        println!();
                    }
                }
                sleep(Duration::from_secs(1));
            }
        });
        hdls.push(hdl);
    }

    let staff = std::thread::spawn(move || {
        for n in 0..4 {
            if n % 2 == 0 {
                let mut guard = gallery.write().unwrap(); // write lock
                guard.clear(); // これはMapの関数。中身を空にする
                guard.insert("ゴッホ", "星月夜");
                guard.insert("エッシャー", "滝");
            } else {
                let mut guard = gallery.write().unwrap(); // write lock
                guard.clear(); // これはMapの関数。中身を空にする
                guard.insert("葛飾北斎", "富嶽三十六景  神奈川沖浪裏");
                guard.insert("ミュシャ", "黄道十二宮");
            }
            sleep(Duration::from_secs(2));
        }
    });

    for hdl in hdls {
        hdl.join().unwrap();
    }
    staff.join().unwrap();

}
