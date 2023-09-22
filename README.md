# osm-local-tile-dl

Download OpenStreetMap-tiles to your disk en-masse.

**Use with absolute caution.** Downloading tiles en-masse can hog
down a tile server easily. I am not responsible for any damage this
tool may cause.

Only localhost and private network HTTP tile servers are supported.

## Usage

1. Setup local tile server with Docker

<https://switch2osm.org/serving-tiles/using-a-docker-container/>

2. Download tiles from the tile server with this tool.

The tool will not download tiles that already exist on disk.
Also parallel downloads are disabled by default. Those
can be enabled by setting `--rate` to a value greater than 1.

## CLI Example

```bash
cargo run --release -- \
  --north 50.811 \
  --east 6.1649 \
  --south 50.7492 \
  --west 6.031 \
  --url http://localhost:8080/\{z\}/\{x\}/\{y\}.png \
  --output ./tiles \
  --rate 10
```

## Library Example
```rust
use osm_local_tile_dl::{fetch, BoundingBox, Config};
use std::path::Path;
use std::time::Duration;

async fn fetch_tiles() {
    let config = Config {
        bounding_box: BoundingBox::new_deg(50.811, 6.1649, 50.7492, 6.031),
        fetch_rate: 10,
        output_folder: Path::new("./tiles"),
        url: "http://localhost:8080/{z}/{x}/{y}.png",
        zoom_level: 10,
    };
    fetch(config).await.expect("failed fetching tiles");
}

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(fetch_tiles());
}
```

## License

MIT
