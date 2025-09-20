use csv_md::CsvTable;

fn main() {
    let about = "ID, Name, Description
1, Language, Rust
2, CSV_MD, Convert CSV to MD
,we,
3, License, MIT
4, Author, Roman Chumak";

    let top = "PID USER PR NI VIRT RES SHR S %CPU %MEM TIME+ COMMAND
470459 p4ymak 20 0 1134,3g 272896 137040 S 20,5 0,8 17:23.19 yandex_browser
8579 p4ymak 20 0 33,4g 360608 186888 S 5,6 1,1 138:08.33 yandex_browser
9686 p4ymak 20 0 1136,3g 338264 137220 S 4,0 1,0 167:05.49 yandex_browser
2416 p4ymak 9 -11 2313640 25368 16036 R 3,0 0,1 99:52.35 pulseaudio
2713 p4ymak 20 0 61,5g 687576 20176 S 2,6 2,1 48:44.43 zellij
352154 p4ymak 20 0 1605320 114120 66380 S 1,3 0,3 20:18.42 roomor
10679 p4ymak 20 0 2705656 508412 162408 S 1,0 1,6 45:36.91 nautilus
442069 p4ymak 20 0 11228 5456 3332 R 1,0 0,0 3:25.83 top
8518 p4ymak 20 0 32,9g 523100 339144 S 0,7 1,6 48:23.82 yandex_browser
994 root 20 0 2164064 25924 4536 S 0,3 0,1 0:46.97 dockerd
1992 p4ymak 20 0 5320664 350936 155396 S 0,3 1,1 205:18.17 gnome-shell
8451 p4ymak 20 0 6690024 869936 212740 S 0,3 2,7 12:07.54 Telegram
8584 p4ymak 20 0 32,4g 139728 105896 S 0,3 0,4 15:20.14 yandex_browser
8600 p4ymak 20 0 32,4g 67112 42340 S 0,3 0,2 2:39.23 yandex_browser
359820 p4ymak 20 0 2612716 46048 24072 S 0,3 0,1 0:01.04 hx
440678 p4ymak 20 0 1132,1g 192616 120536 S 0,3 0,6 0:21.15 yandex_browser
1 root 20 0 23364 14240 9776 S 0,0 0,0 0:13.27 systemd
2 root 20 0 0 0 0 S 0,0 0,0 0:00.14 kthreadd
3 root 0 -20 0 0 0 I 0,0 0,0 0:00.00 rcu_gp
4 root 0 -20 0 0 0 I 0,0 0,0 0:00.00 rcu_par_gp
5 root 0 -20 0 0 0 I 0,0 0,0 0:00.00 slub_flushwq
6 root 0 -20 0 0 0 I 0,0 0,0 0:00.00 netns";

    println!("\n{}\n\n{}", about, CsvTable::new(',', about).as_md());

    // println!("\n{}\n\n{}", top, CsvTable::new(' ', top).as_md());
    let sql = "oid|relname|relnamespace|reltype|reloftype|relowner|relam|relfilenode|reltablespace|relpages|reltupl
es|relallvisible|reltoastrelid|relhasindex|relisshared|relpersistence|relkind|relnatts|relchecks|relhasrules|relhastriggers|relhassubclass|relrowsecurity|relfor
cerowsecurity|relispopulated|relreplident|relispartition|relrewrite|relfrozenxid|relminmxid|relacl|reloptions|relpartbound\n16394|t1|2200|16396|0|10|2|16394|0|0
|-1|0|0|false|false|p|r|0|0|false|false|false|false|false|true|d|false|0|870|1\n16397|t2|2200|16399|0|10|2|16397|0|0|-1|0|0|false|false|p|r|0|0|false|false|fals
e|false|false|true|d|false|0|872|1\n16400|t3|2200|16402|0|10|2|16400|0|0|-1|0|0|false|false|p|r|0|0|false|false|false|false|false|true|d|false|0|874|1\n16403|tb
|2200|16405|0|10|2|16403|0|444|100150|444|0|false|false|p|r|1|0|false|false|false|false|false|true|d|false|0|878|1\n2619|pg_statistic|11|10029|0|10|2|2619|0|21|
453|21|2840|true|false|p|r|31|0|false|false|false|false|false|true|n|false|0|826|1|[\"qhb=arwdDxt/qhb\"]\n1247|pg_type|11|71|0|10|2|0|0|17|697|17|4171|true|fals
e|p|r|32|0|false|false|false|false|false|true|n|false|0|826|1|[\"qhb=arwdDxt/qhb\",\"=r/qhb\"]\n2836|pg_toast_1255|99|0|0|10|2|0|0|6|12|6|0|true|false|p|t|3|0|f
alse|false|false|false|false|true|n|false|0|826|1\n2837|pg_toast_1255_index|99|0|0|10|403|0|0|2|12|0|0|false|false|p|i|2|0|false|false|false|false|false|true|n|
false|0|0|0\n4171|pg_toast_1247|99|0|0|10|2|0|0|0|0|0|0|true|false|p|t|3|0|false|false|false|false|false|true|n|false|0|826|1";
    println!("\n{}", CsvTable::new('|', sql).as_md());
}
