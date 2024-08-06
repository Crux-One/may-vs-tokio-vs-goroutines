use std::{
    fs::File,
    io::{self, BufRead, BufReader, Cursor, Read},
    sync::Arc,
    time::Instant,
};
use tokio::sync::mpsc;
use tokio::task;
use zip::read::{ZipArchive, ZipFile};

#[tokio::main]
async fn main() -> io::Result<()> {
    let zip_path: &str = "../target.zip";
    let dict_path: &str = "../xato-net-10-million-passwords.txt";
    let num_workers: usize = 10;

    let mut zip_file = File::open(zip_path)?;
    let mut zip_data = Vec::new();
    zip_file.read_to_end(&mut zip_data)?;

    let file = File::open(dict_path)?;
    let reader = BufReader::new(file);
    let passwords: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let passwords_per_worker = passwords.len() / num_workers;

    // println!("Num of passwords: {}", passwords.len());
    // println!("Num of workers: {}", num_workers);
    // println!("Num of passwords / worker: {}", passwords_per_worker);

    let (result_sender, mut result_receiver) = mpsc::channel(num_workers);
    let zip_data = Arc::new(zip_data);

    let start = Instant::now();

    for i in 0..num_workers {
        let start_idx = i * passwords_per_worker;
        let end_idx = start_idx + passwords_per_worker;
        if start_idx >= passwords.len() {
            break;
        }
        let end_idx = if end_idx > passwords.len() {
            passwords.len()
        } else {
            end_idx
        };

        let zip_data_clone = Arc::clone(&zip_data);
        let passwords_chunk = passwords[start_idx..end_idx].to_vec();
        let result_sender_clone = result_sender.clone();

        task::spawn(async move {
            worker(zip_data_clone, passwords_chunk, result_sender_clone).await;
        });
    }

    drop(result_sender);

    if let Some(password) = result_receiver.recv().await {
        // println!("Password found: {}", password);
    } else {
        // println!("Password not found");
    }

    let duration = start.elapsed();
    let duration_ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
    println!("{}", duration_ms); // ms

    Ok(())
}

async fn worker(
    zip_data: Arc<Vec<u8>>,
    passwords: Vec<String>,
    result_sender: mpsc::Sender<String>,
) {
    let mut r = Cursor::new(zip_data.to_vec());

    for password in passwords {
        if try_password(&mut r, &password) {
            let _ = result_sender.send(password).await;
            return;
        }
    }
}

fn try_password<R: Read + io::Seek>(r: &mut R, password: &str) -> bool {
    let mut zip = match ZipArchive::new(r) {
        Ok(zip) => zip,
        Err(_) => return false,
    };

    for i in 0..zip.len() {
        let mut file: ZipFile = match zip.by_index_decrypt(i, password.as_bytes()) {
            Ok(file) => file,
            Err(_) => return false,
        };

        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_ok() {
            return true;
        }
    }

    false
}
