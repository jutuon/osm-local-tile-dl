use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version,
    AppSettings, Arg,
};
use std::{error::Error, f64, path::Path, str::FromStr};

use osm_tile_downloader::*;

const BBOX_NORTH_ARG: &str = "BBOX_NORTH";
const BBOX_SOUTH_ARG: &str = "BBOX_SOUTH";
const BBOX_WEST_ARG: &str = "BBOX_WEST";
const BBOX_EAST_ARG: &str = "BBOX_EAST";
const OUTPUT_ARG: &str = "OUTPUT";
const PARALLEL_FETCHES_ARG: &str = "PARALLEL_FETCHES";
const UP_TO_ZOOM_ARG: &str = "UP_TO_ZOOM";
const URL_ARG: &str = "URL_ARG";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    fn is_numeric<T: FromStr>(v: String) -> Result<(), String> {
        v.parse::<T>()
            .map(|_| ())
            .map_err(|_| "must be numeric".to_owned())
    }
    fn is_geo_coord(v: String) -> Result<(), String> {
        let val = v.parse::<f64>().map_err(|_| "must be numeric".to_owned())?;

        if val < 0f64 {
            return Err("must be >= 0°".to_owned());
        } else if val >= 360f64 {
            return Err("must be < 360°".to_owned());
        }

        Ok(())
    }

    let matches = app_from_crate!()
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name(BBOX_NORTH_ARG)
                .help("Latitude of north bounding box boundary (in degrees)")
                .validator(is_geo_coord)
                .required(true)
                .takes_value(true)
                .short("n")
                .long("north"),
        )
        .arg(
            Arg::with_name(BBOX_SOUTH_ARG)
                .help("Latitude of south bounding box boundary (in degrees)")
                .validator(is_geo_coord)
                .required(true)
                .takes_value(true)
                .short("s")
                .long("south"),
        )
        .arg(
            Arg::with_name(BBOX_EAST_ARG)
                .help("Longitude of east bounding box boundary (in degrees)")
                .validator(is_geo_coord)
                .required(true)
                .takes_value(true)
                .short("e")
                .long("east"),
        )
        .arg(
            Arg::with_name(BBOX_WEST_ARG)
                .help("Longitude of west bounding box boundary (in degrees)")
                .validator(is_geo_coord)
                .required(true)
                .takes_value(true)
                .short("w")
                .long("west"),
        )
        .arg(
            Arg::with_name(PARALLEL_FETCHES_ARG)
                .help("The amount of tiles fetched in parallel.")
                .validator(|v| match v.parse::<u8>() {
                    Ok(v) => {
                        if v > 0 {
                            Ok(())
                        } else {
                            Err("must be > 0".to_owned())
                        }
                    }
                    Err(_) => Err("must be numeric".to_owned()),
                })
                .default_value("5")
                .takes_value(true)
                .short("r")
                .long("rate"),
        )
        .arg(
            Arg::with_name(UP_TO_ZOOM_ARG)
                .help("The maximum zoom level to fetch")
                .validator(is_numeric::<u8>)
                .default_value("18")
                .takes_value(true)
                .short("z")
                .long("zoom"),
        )
        .arg(
            Arg::with_name(OUTPUT_ARG)
                .help("The folder to output the tiles to. May contain format specifiers (and subfolders) to specify how the files will be laid out on disk.")
                .default_value("output")
                .takes_value(true)
                .short("o")
                .long("output"),
        )
        .arg(
            Arg::with_name(URL_ARG)
                .help("The URL with format specifiers `{x}`, `{y}`, `{z}` to fetch the tiles from")
                .required(true)
                .takes_value(true)
                .short("u")
                .long("url")
        )
        .get_matches();

    let config = Config {
        bounding_box: BoundingBox::new_deg(
            matches.value_of(BBOX_NORTH_ARG).unwrap().parse().unwrap(),
            matches.value_of(BBOX_EAST_ARG).unwrap().parse().unwrap(),
            matches.value_of(BBOX_SOUTH_ARG).unwrap().parse().unwrap(),
            matches.value_of(BBOX_WEST_ARG).unwrap().parse().unwrap(),
        ),
        fetch_rate: matches
            .value_of(PARALLEL_FETCHES_ARG)
            .unwrap()
            .parse()
            .unwrap(),
        output_folder: Path::new(matches.value_of(OUTPUT_ARG).unwrap()),
        url: matches.value_of(URL_ARG).unwrap(),
        zoom_level: matches.value_of(UP_TO_ZOOM_ARG).unwrap().parse().unwrap(),
    };

    fetch(config).await?;
    Ok(())
}