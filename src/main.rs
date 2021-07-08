use youtube_dl::{YoutubeDl, YoutubeDlOutput};

fn main() {
    let swamp = match YoutubeDl::new("https://www.youtube.com/watch?v=psFzJv8g6jc")
        .socket_timeout("15")
        .run()
    {
        Ok(a) => a,
        _ => unreachable!(),
    };
    let video = match swamp {
        YoutubeDlOutput::Playlist(_) => unreachable!(),
        YoutubeDlOutput::SingleVideo(v) => v,
    };
    dbg!(&video.formats.unwrap()[0].url.as_ref().unwrap());
}
