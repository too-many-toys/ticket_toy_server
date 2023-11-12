use mongodb::bson::{oid::ObjectId, Bson};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Movie {
    pub _id: Option<ObjectId>,
    pub id: i64,
    pub adult: bool,
    pub backdrop_path: String,
    pub belongs_to_collection: Option<BelongsToCollection>,
    pub budget: i64,
    pub genres: Vec<Genre>,
    pub homepage: String,
    pub imdb_id: String,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: String,
    pub production_companies: Vec<ProductionCompany>,
    pub production_countries: Vec<ProductionCountry>,
    pub release_date: String,
    pub revenue: i64,
    pub runtime: i64,
    pub spoken_languages: Vec<SpokenLanguage>,
    pub status: String,
    pub tagline: String,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: i64,
}

impl From<Movie> for Bson {
    fn from(movie: Movie) -> Bson {
        mongodb::bson::to_bson(&movie).unwrap()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ProductionCompany {
    pub id: i64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ProductionCountry {
    pub iso_3166_1: String,
    pub name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SpokenLanguage {
    pub english_name: String,
    pub iso_639_1: String,
    pub name: String,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BelongsToCollection {
    pub id: i64,
    pub name: String,
    pub poster_path: String,
    pub backdrop_path: String,
}
