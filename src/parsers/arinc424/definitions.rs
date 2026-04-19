pub const BLANK: u8 = b' ';

#[derive(Debug, PartialEq, Eq)]
pub enum RecordType {
    Standard,
    Tailored,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AreaCode {
    Africa,
    Canada,
    EasternEurope,
    Europe,
    LatinAmerica,
    MiddleEast,
    Pacific,
    SouthAmerica,
    SouthPacific,
    USA,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NavDatabaseMajorSection {
    MORA,
    Navaid,
    Enroute,
    Heliport,
    Airport,
    CompanyRoutes,
    Tables,
    Airspace,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NavDatabaseSubsection {
    GridMORA,
    VHFNavaid,
    NDBNavaid,
    TACANDuplicates,
    Waypoints,
    AirwayMarkers,
    HoldingPatterns,
    AirwaysAndRoutes,
    SpecialActivityAreas,
    PreferredRoutes,
    AirwayRestrictions,
    Communications,
    ReferencePoints,
    TerminalWaypoints,
    SIDS,
    STARS,
    ApproachProcedures,
    Helipads,
    TAA,
    MSA,
    SBASPathPoint,
    Gates,
    Runways,
    LocalizerGlideslope,
    MLS,
    LocalizerMarker,
    TerminalNDB,
    GBASPathPoint,
    FlightPlanningARRDEP,
    GLSStation,
    CompanyRoutes,
    AlternateRecords,
    HelicopterOperationRoutes,
    CruisingTables,
    GeographicalReference,
    ATNData,
    CommunicationType,
    ControlledAirspace,
    FIRUIR,
    RestrictiveAirspace,
}
