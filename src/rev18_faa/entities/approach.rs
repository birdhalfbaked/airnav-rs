use crate::rev18_faa::definitions::{
    AirportHeliportApproachRouteType, RequiredNavigationPerformance, RouteDistanceFrom,
};
use crate::rev18_faa::entities::common::{Distance, NavaidReference};

#[derive(Debug, PartialEq)]
pub enum ApproachDomain {
    Airport,
    Heliport,
}

#[derive(Debug, PartialEq)]
pub enum PathTurnType {
    Left,
    Right,
    None,
}

#[derive(Debug, PartialEq)]
pub enum LegType {
    /// IF - Initial Fix
    InitialFix,
    /// TF - Track To Fix
    TrackToFix,
    /// CF - Course To Fix
    CourseToFix,
    /// DF - Direct to Fix
    DirectToFix,
    /// FA - Fix to an altitude
    FixToAltitude,
    /// FC - Track from a fix for some distance
    TrackFromFix,
    /// FD - Track from a fix to DME distance
    TrackFromFixToDmeDistance,
    /// FM - Track from a fix to manual termination
    TrackFromFixToManualTermination,
    ///CA - Course to an altitude
    CourseToAltitude,
    /// CD - Course from a fix to DME distance
    CourseFromFixToDmeDistance,
    /// CI - Course to an intercept
    CourseToIntercept,
    /// CR - Course to a radial termination
    CourseToRadial,
    /// RF - Constant radius arc
    ConstantRadiusArc,
    /// AF - Arc to a Fix
    ArcToFix,
    /// VA - Heading to an altitude
    HeadingToAltitude,
    /// VD - Heading from a fix to DME distance
    HeadingFromFixToDmeDistance,
    /// VI - Heading to an intercept
    HeadingToIntercept,
    /// VM - Heading to a manual termination
    HeadingToManualTermination,
    /// VR - Heading to a radial termination
    HeadingToRadial,
    /// PI - Procedure Turn
    ProcedureTurn,
    /// HA - Holding in Lieu to altitude
    HoldingInLieuToAltitude,
    /// HF - Holding in Lieu, single circuit
    HoldingInLieuSingleCircuit,
    /// HM - Holding in Lieu, manual termination
    HoldingInLieuManualTermination,
}

impl std::fmt::Display for LegType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InitialFix => "Initial Fix",
                Self::TrackToFix => "Track To Fix",
                Self::CourseToFix => "Course To Fix",
                Self::DirectToFix => "Direct To Fix",
                Self::FixToAltitude => "Fix To Altitude",
                Self::TrackFromFix => "Track From Fix",
                Self::TrackFromFixToDmeDistance => "Track From Fix To DME Distance",
                Self::TrackFromFixToManualTermination => "Track From Fix To Manual Termination",
                Self::CourseToAltitude => "Course To Altitude",
                Self::CourseFromFixToDmeDistance => "Course From Fix To DME Distance",
                Self::CourseToIntercept => "Course To Intercept",
                Self::CourseToRadial => "Course To Radial",
                Self::ConstantRadiusArc => "Constant Radius Arc",
                Self::ArcToFix => "Arc To Fix",
                Self::HeadingToAltitude => "Heading To Altitude",
                Self::HeadingFromFixToDmeDistance => "Heading From Fix To DME Distance",
                Self::HeadingToIntercept => "Heading To Intercept",
                Self::HeadingToRadial => "Heading To Radial",
                Self::HeadingToManualTermination => "Heading To Manual Termination",
                Self::ProcedureTurn => "Procedure Turn",
                Self::HoldingInLieuToAltitude => "Holding In Lieu To Altitude",
                Self::HoldingInLieuSingleCircuit => "Holding In Lieu Single Circuit",
                Self::HoldingInLieuManualTermination => "Holding In Lieu Manual Termination",
            }
        )
    }
}

#[derive(Debug)]
pub struct ApproachReference {
    pub identifier: Box<str>,
    pub icao_code: Box<str>,
    pub airport_identifier: Option<Box<str>>,
    pub airport_icao_code: Option<Box<str>>,
}

#[derive(Debug)]
pub struct MSAReference {
    pub center_fix: Box<str>,
    pub center_fix_icao_code: Box<str>,
    pub section: Box<str>,
    pub subsection: Box<str>,
    pub multiple_code: Box<str>,
}

#[derive(Debug)]
pub struct TAAReference {
    pub sector_identifier: Box<str>,
    pub procedure_turn_required: bool,
}

#[derive(Debug)]
pub struct ArcData {
    pub navaid: NavaidReference,
    pub arc_radius: f64,
    pub theta: f64,
    pub rho: f64,
}

pub struct CategoryTableData {
    pub decision_height: f64,
    pub minimum_descent_altitude: f64,
}

pub struct ApproachCategoryTable {
    pub category_a: Option<CategoryTableData>,
    pub category_b: Option<CategoryTableData>,
    pub category_c: Option<CategoryTableData>,
    pub category_d: Option<CategoryTableData>,
}

pub struct ApproachLegData {
    pub sequence_number: u64,
    pub fix_identifier: Box<str>,
    pub fix_icao_code: Box<str>,
    pub leg_type: LegType,
    pub rnp: RequiredNavigationPerformance,
    pub turn_direction: PathTurnType,
    pub course: Option<f64>,
    pub distance: Option<Distance>,
    pub arc_data: Option<ArcData>,
    pub msa_reference: Option<MSAReference>,
    pub taa_reference: Option<TAAReference>,
}

pub struct ApproachTransitionData {
    pub transition_type: AirportHeliportApproachRouteType,
    pub transition_identifier: Box<str>,
}

pub struct Approach {
    pub domain: ApproachDomain,
    pub reference: ApproachReference,
    pub category_table: ApproachCategoryTable,
}
