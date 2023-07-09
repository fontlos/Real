// use rodio::{Decoder, OutputStream, Sink};
// use std::fs::File;
// use std::io::BufReader;
// use std::thread;

// struct PlayerConfig {
//     url: String,
// }

// impl PlayerConfig {
//     pub fn play(self) {
//         thread::spawn(|| {
//             // 获取默认物理声音设备的输出流句柄
//             let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//             // 新建音轨
//             let sink = Sink::try_new(&stream_handle).unwrap();
//             // 读取文件
//             let file = BufReader::new(File::open(self.url).unwrap());
//             // 将音频文件解码为 Source
//             let source = Decoder::new(file).unwrap();
//             // 将源添加到音轨
//             sink.append(source);
//             sink.sleep_until_end();
//         });
//     }
// }
