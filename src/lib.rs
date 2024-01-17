use std::fmt::Display;
use std::time::Instant;
use fake::{Dummy, Faker};
use fake::faker::name::raw::{Name, FirstName, LastName};
use fake::faker::company::raw::CompanyName;
use fake::locales::EN;

pub fn get_match() -> Game {
    Game::dummy(&Faker)
}


#[derive(Dummy, Default, Debug)]
pub struct Game {
    pub competitor_one: Competitor,
    pub competitor_two: Competitor,
    pub match_setup: MatchSetup,
    pub match_details: MatchDetails,
}

#[derive(Dummy, Default, Debug)]
pub struct MatchDetails {
    pub competitor_one: CompetitorScore,
    pub competitor_two: CompetitorScore,
    pub match_timer: MatchTimer
}

#[derive(Dummy, Default, Debug)]
pub struct MatchTimer {
    #[dummy(faker = "5..10")]
    duration_minutes: u32,

    #[dummy(default)]
    elapsed_milliseconds: u32,

    #[dummy(default)]
    started_at: Option<Instant>,

    #[dummy(default)]
    running: bool
}

#[derive(Dummy, Default, Debug)]
pub struct CompetitorScore {
    #[dummy(default)]
    pub points: u32,

    #[dummy(default)]
    pub advantages: u32,

    #[dummy(default)]
    pub penalties: u32,

    #[dummy(default)]
    pub medical: u32
}

pub enum Score {
    Point(u32),
    Advantage(u32),
    Penalty(u32),
    Medical(u32)
}

#[derive(Dummy, Default, Debug)]
pub struct MatchSetup {
    #[dummy(faker = "5..10")]
    pub duration_minutes: u32,
    pub match_type: MatchType,
    pub sport_type: SportType,
    pub gender: Gender,
    pub division_name: String,
    pub weight_class: String
}

#[derive(Dummy, Default, Debug)]
pub enum SportType {
    #[default]
    Gi,
    NoGi
}

#[derive(Dummy, Default, Debug)]
pub enum Gender {
    #[default]
    Male,
    Female,
    Mixed
}

#[derive(Dummy, Default, Debug)]
pub enum MatchType {
    #[default]
    Heat,
    QuarterFinal,
    SemiFinal,
    Final
}

#[derive(Debug, Dummy, Default)]
pub struct Competitor {
    #[dummy(faker = "FirstName(EN)")]
    pub first_name: String,

    #[dummy(faker = "LastName(EN)")]
    pub last_name: String,

    pub country: Country,

    pub team: Team
}
impl Competitor {
    pub fn display_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[derive(Debug, Dummy, Default)]
pub struct Team {
    #[dummy(faker = "CompanyName(EN)")]
    pub name: String,

    #[dummy(default)]
    pub logo: String
}

#[derive(Dummy, Debug, Default)]
pub enum Country {
    UnitedArabEmirates,
    Argentina,
    #[default]
    Australia,
    Belgium,
    Brazil,
    Canada,
    Germany,
    Denmark,
    Spain,
    Finland,
    France,
    Ireland,
    Morocco,
    Norway,
    Philippines,
    NewZealand,
    Japan,
    UnitedStates,
}

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Country::UnitedArabEmirates => "ARE",
            Country::Argentina => "ARG",
            Country::Australia => "AUS",
            Country::Belgium => "BEL",
            Country::Brazil => "BRA",
            Country::Canada => "CAN",
            Country::Germany => "DEU",
            Country::Denmark => "DNK",
            Country::Spain => "ESP",
            Country::Finland => "FIN",
            Country::France => "FRA",
            Country::Ireland => "IRL",
            Country::Morocco => "MAR",
            Country::Norway => "NOR",
            Country::Philippines => "PHL",
            Country::NewZealand => "NZL",
            Country::Japan => "JPN",
            Country::UnitedStates => "USA"
        };
        write!(f, "{}", s)
    }
}