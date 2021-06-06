use anyhow::Result; // TODO
use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ojigineko {
    is_sleeping: bool,
    location: usize,
    gone: bool, // :cry:
    #[serde(with = "chrono::serde::ts_milliseconds")]
    updated_at: DateTime<Utc>,
    #[serde(skip)]
    activity: &'static str,
}

impl Default for Ojigineko {
    fn default() -> Self {
        Self {
            is_sleeping: false,
            location: 0,
            gone: false,
            updated_at: Utc::now(),
            activity: "",
        }
    }
}

impl Ojigineko {
    const ACTIVITY_SLEEPING: &'static str = "ojigineko-sleeping";
    const ACTIVITIES: [&'static str; 16] = [
        "dot-ojigineko",
        "harassment-ojigineko",
        "nameraka-ojigineko",
        "nameraka-ojigineko-extreme-fast",
        "ojigineko",
        "ojigineko-extremefast",
        "ojigineko-fast",
        "ojigineko-hd",
        "ojigineko-mirror",
        "ojigineko-muscle-exercise",
        "ojigineko-sleeping",
        "ojigineko-superfast",
        "ojigineko-upside-down",
        "ojigineko-waking",
        "party-ojigineko",
        "tosshutsu-symmetry-ojigineko",
    ];

    fn step(&mut self, cur: DateTime<Utc>) {
        if self.gone {
            return;
        }

        let mut rng = rand::thread_rng();
        use rand::Rng;
        let rnd = rng.gen_range(0..100);

        if self.is_sleeping {
            let wake = match cur.hour() {
                6 => rnd < 5,
                7 => rnd < 10,
                8 => rnd < 20,
                9 => rnd < 40,
                10 => rnd < 70,
                11 => rnd < 90,
                12 => rnd < 100,
                _ => false,
            };
            if wake {
                self.is_sleeping = false;
            }
        } else {
            let sleep = match cur.hour() {
                21 => rnd < 5,
                22 => rnd < 10,
                23 => rnd < 20,
                0 => rnd < 40,
                1 => rnd < 70,
                2 => rnd < 90,
                3 => rnd < 100,
                _ => false,
            };
            if sleep {
                self.is_sleeping = true;
            }
        }

        if !self.is_sleeping {
            self.location =
                num::clamp(self.location as isize + rng.gen_range(-2..=2), 0, 16) as usize;
        }

        self.activity = if self.is_sleeping {
            Self::ACTIVITY_SLEEPING
        } else {
            let idx = rng.gen_range(0..16);
            Self::ACTIVITIES[idx]
        };
    }

    pub fn forward(&mut self, until: DateTime<Utc>) -> Option<()> {
        println!("{:?}", self);
        let hr: Duration = Duration::hours(1); // FIXME const

        let mut cur = self.updated_at;
        let mut res = None;
        while cur + hr < until {
            self.step(cur);
            cur = cur + hr;
            res = Some(());
        }
        self.updated_at = cur;

        res
    }

    /// Call to_text only after `forward` returned `Some(_)`.
    pub fn to_text(&self) -> String {
        ":void:".repeat(self.location) + &format!(":{}:", self.activity)
    }
}

pub struct Store {
    path: std::path::PathBuf,
    ojigineko: Ojigineko,
}

impl Store {
    pub fn new(path: std::path::PathBuf) -> Result<Self> {
        let ojigineko = Self::load(&path).unwrap_or_default(); // TODO errors other than NoSuchFile
        Ok(Self { path, ojigineko })
    }

    pub fn load(path: impl AsRef<std::path::Path>) -> Result<Ojigineko> {
        use std::fs::File;
        use std::io::BufReader;

        let file = File::open(path)?;
        let r = BufReader::new(file);

        Ok(serde_json::from_reader(r)?)
    }

    pub fn ojigineko(&mut self) -> &mut Ojigineko {
        &mut self.ojigineko
    }
}

impl Drop for Store {
    fn drop(&mut self) {
        if let Err(err) = (|| -> Result<()> {
            use std::fs::File;
            use std::io::BufWriter;

            let file = File::create(&self.path)?;
            let w = BufWriter::new(file);

            Ok(serde_json::to_writer(w, &self.ojigineko)?)
        })() {
            eprintln!(
                "<ojigineko-life::Store as Drop>::drop: err during save...: {}",
                err
            );
        };
    }
}
