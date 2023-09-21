use anyhow::Result;
use clap::Parser;
use std::{f64, num::NonZeroU8, path::PathBuf, str::FromStr};

use osm_local_tile_dl::*;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
struct Cli {
    /// Latitude of north bounding box boundary (in degrees)
    #[arg(short, long, value_name = "BBOX_NORTH")]
    pub north: BoundingBoxValue,

    /// Latitude of south bounding box boundary (in degrees)
    #[arg(short, long, value_name = "BBOX_SOUTH")]
    pub south: BoundingBoxValue,

    /// Longitude of east bounding box boundary (in degrees)
    #[arg(short, long, value_name = "BBOX_EAST")]
    pub east: BoundingBoxValue,

    /// Longitude of west bounding box boundary (in degrees)
    #[arg(short, long, value_name = "BBOX_WEST")]
    pub west: BoundingBoxValue,

    /// The amount of tiles fetched in parallel.
    #[arg(short, long, value_name = "PARALLEL_FETCHES", default_value = "5")]
    pub rate: NonZeroU8,

    /// The maximum zoom level to fetch
    #[arg(short, long, value_name = "UP_TO_ZOOM", default_value = "18")]
    pub zoom: u8,

    /// The folder to output the tiles to. May contain format specifiers (and subfolders) to specify how the files will be laid out on disk.
    #[arg(short, long, value_name = "OUTPUT_DIR")]
    pub output: PathBuf,

    /// The URL with format specifiers `{x}`, `{y}`, `{z}` to fetch the tiles from.
    #[arg(short, long, value_name = "URL")]
    pub url: String,
}

#[derive(Debug, Clone, Copy)]
struct BoundingBoxValue {
    value: f64,
}

impl FromStr for BoundingBoxValue {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, String> {
        let value = value
            .parse::<f64>()
            .map_err(|_| "must be numeric".to_owned())?;

        if value < 0f64 {
            return Err("must be >= 0°".to_owned());
        } else if value >= 360f64 {
            return Err("must be < 360°".to_owned());
        }

        Ok(Self { value })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Config {
        bounding_box: BoundingBox::new_deg(
            cli.north.value,
            cli.east.value,
            cli.south.value,
            cli.west.value,
        ),
        fetch_rate: cli.rate.get(),
        output_folder: &cli.output,
        url: &cli.url,
        zoom_level: cli.zoom,
    };

    fetch(config).await?;
    Ok(())
}
