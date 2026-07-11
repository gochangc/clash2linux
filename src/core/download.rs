use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::time::Duration;

pub fn download_text(url: &str) -> Result<String> {
    let body = ureq::get(url)
        .set("User-Agent", "clash2linux/0.1.0")
        .call()
        .with_context(|| format!("下载失败: {}", url))?
        .into_string()
        .with_context(|| format!("读取响应失败: {}", url))?;
    Ok(body)
}

pub fn download_file(url: &str, dest: &Path) -> Result<()> {
    let response = ureq::get(url)
        .set("User-Agent", "clash2linux/0.1.0")
        .call()
        .with_context(|| format!("下载失败: {}", url))?;

    let total_size = response.header("Content-Length").unwrap_or("0").parse::<u64>().unwrap_or(0);

    let pb = if total_size > 0 {
        let bar = ProgressBar::new(total_size);
        let style = ProgressStyle::default_bar()
            .template("{bar:40.cyan/blue}")
            .unwrap()
            .progress_chars("#>-");
        bar.set_style(style);
        bar.enable_steady_tick(Duration::from_millis(100));
        bar
    } else {
        ProgressBar::hidden()
    };

    fs::create_dir_all(dest.parent().unwrap_or(Path::new("/")))?;
    let mut file = File::create(dest)?;
    let mut reader = response.into_reader();
    let mut wrapper = ProgressWriter::new(&mut file, &pb);
    io::copy(&mut reader, &mut wrapper).context("复制数据失败")?;
    pb.finish_and_clear();
    Ok(())
}

struct ProgressWriter<'a, W: Write> {
    inner: &'a mut W,
    pb: &'a ProgressBar,
}

impl<'a, W: Write> ProgressWriter<'a, W> {
    fn new(inner: &'a mut W, pb: &'a ProgressBar) -> Self {
        Self { inner, pb }
    }
}

impl<'a, W: Write> Write for ProgressWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;
        self.pb.inc(n as u64);
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

pub fn decompress_gz(src: &Path, dest: &Path) -> Result<()> {
    let file = File::open(src).with_context(|| format!("打开文件失败: {:?}", src))?;
    let mut decoder = GzDecoder::new(file);
    let mut out = File::create(dest).with_context(|| format!("创建文件失败: {:?}", dest))?;
    io::copy(&mut decoder, &mut out).with_context(|| "解压 .gz 失败")?;
    Ok(())
}
