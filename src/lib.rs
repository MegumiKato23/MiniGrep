use std::path::{Path, PathBuf};
use tokio::io::{BufReader, AsyncBufReadExt}; 
use tokio::fs::File;
use walkdir::WalkDir;
use std::sync::Arc;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    // 要搜索的目标字符串
    pub pattern: String,
    
    // 搜索目录（默认为当前目录）目录位置
    #[clap(default_value = ".")]
    pub path: String,
}

struct MatchResult {
    path: PathBuf,
    lines: Vec<(usize, String)>,
}

pub async fn run(
    path: String,
    pattern: String,
) -> anyhow::Result<()>{
    // 使用并行迭代器遍历目录
    let files: Vec<PathBuf> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_owned())
        .collect();

    let pattern = Arc::new(pattern);
    let mut tasks = FuturesUnordered::new();

    for file in files {
        let pattern = Arc::clone(&pattern);
        
        tasks.push(tokio::spawn(async move {
            if file.is_file() {
                if let Some(res) = search(&file, &pattern).await {
                    println!("\n匹配文件: {}", res.path.display());
                    for (num, line) in res.lines {
                        println!("{:4} {}", num, line);
                    }
                }
            }
        }))
    }

    // 等待所有任务完成
    while let Some(result) = tasks.next().await {
        result?; 
    }

    Ok(())
}

async fn search(
    path: &Path,
    pattern: &str,
) -> Option<MatchResult> {
    let file = File::open(path).await.ok()?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut matches = Vec::new();
    let mut line_num = 0;

    while let Ok(Some(line)) = lines.next_line().await {
        line_num += 1;
        let has_match = line.contains(pattern);

        if has_match {
            matches.push((line_num, line.clone()));
        }
    }

    (!matches.is_empty()).then(|| MatchResult {
        path: path.to_path_buf(),
        lines: matches,
    })
}