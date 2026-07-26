//! # ARINC 424-18 FAA Extension
//! This module contains the parser for the ARINC 424 Revision 18 standard.
//!
//! More context is that the FAA applies the following exceptions to the standard:
//!
//! # CIFP Record Types and Coding Notes
//!
//! This document summarizes the record types and coding conventions used in the **Coded Instrument Flight Procedures (CIFP)** dataset.
//!
//! ---
//!
//! # Record Types
//!
//! The CIFP currently provides the following record types:
//!
//! | Record Type | Code(s) |
//! |-------------|---------|
//! | Airports | PA |
//! | Heliports | HA |
//! | Runways | PG |
//! | VHF Navaids | D |
//! | NDB Navaids | DB |
//! | Terminal Navaids | PN |
//! | Localizer and Glide Slope | PI |
//! | Path Points (Primary & Continuation) | PP |
//! | Minimum Safe Altitude (MSA) | PS, HS |
//! | Enroute Waypoints | EA |
//! | Terminal Waypoints | PC, HC |
//! | Standard Instrument Departures (SIDs) | PD |
//! | Standard Terminal Arrival Routes (STARs) | PE |
//! | Approaches (including Level of Service continuation) | PF, HF |
//! | Airways | ER |
//! | Class B/C/D Airspace | UC |
//! | Special Use Airspace (Primary & Continuation) | UR |
//! | Grid MORA | AS |
//!
//! ---
//!
//! # Airports and Heliports
//!
//! - The published **ICAO Airport Identifier** is used whenever available.
//! - If no ICAO identifier exists, the published FAA Airport Identifier is used.
//! - The **IATA Code** field contains the FAA Airport Identifier.
//! - If the airport identifier is four characters long, the IATA field is left blank.
//! - The **Longest Runway (5.54)** field does **not always represent the longest hard-surface runway** at the airport.
//!
//! ---
//!
//! # Runways
//!
//! - Runway suffixes include:
//!   - `W` — Water
//!   - `S` — Soft surface
//!   - `G` — Glider
//!   - `U` — Ultralight
//!   - Numeric suffixes — Assault strips
//! - Non-numeric runway identifiers are included without the `RW` prefix.
//! - Runway Gradient (5.212) and Ellipsoid Height (5.225) are included when available.
//! - If no magnetic variation is available, Runway Magnetic Bearings (5.58) are calculated using the **World Magnetic Model (WMM)**.
//!
//! ---
//!
//! # Waypoints and Fixes
//!
//! ## Supported Domestic Fix Types
//!
//! - Reporting Point
//! - Waypoint
//! - RNAV Waypoint
//! - VFR Waypoint
//! - Computer Navigation
//! - Terminal Waypoint
//! - ATC Coordination
//! - GPS Waypoint
//! - Military Reporting Point
//! - Military Waypoint
//! - NRS Waypoint
//! - Radar
//!
//! ### General Rules
//!
//! - Waypoints with **all-numeric identifiers are not included**.
//! - Offshore fixes may use:
//!   - Customer/Area Code: `USA`
//!   - ICAO Code: `K ` or `P ` (character followed by blank)
//!
//! ### Waypoint Type (5.42)
//!
//! | Value | Meaning |
//! |--------|---------|
//! | R | Ground-based |
//! | W | Satellite-based |
//! | C | Both ground and satellite |
//!
//! Radar-only fixes are coded as `R`.
//!
//! ### Waypoint Name Format Indicator
//!
//! - Field 5.196 is not populated.
//!
//! ### PC Records
//!
//! PC records may be used for named terminal waypoints when:
//!
//! - Used at only one airport
//! - Not used on an enroute airway
//!
//! Otherwise EA records are used.
//!
//! Some unnamed terminal waypoints also use PC records.
//!
//! PC waypoints:
//!
//! - Carry the Customer/Area Code of the parent airport.
//! - Retain their own ICAO Code even when different from the parent airport.
//!
//! ---
//!
//! # Navaids
//!
//! ## PN vs DB
//!
//! A PN record may be used for an NDB when:
//!
//! - Used at only one airport
//! - Not used on an enroute airway
//! - Assigned a five-letter name
//!
//! Otherwise a DB record is used.
//!
//! ## Additional Rules
//!
//! - DME-only facilities have magnetic variation computed using WMM.
//! - If a VOR frequency is unavailable:
//!   - VOR Frequency field = `00000`
//! - Other unavailable frequencies are left blank.
//!
//! ### Navaid Class 3 (5.35)
//!
//! | Code | Meaning |
//! |------|---------|
//! | H | High |
//! | L | Low |
//! | T | Terminal |
//! | U | Undetermined |
//!
//! ### Navaid Class 5
//!
//! `N` indicates a VORTAC when VOR and TACAN coordinates differ by **0.1 NM or more**.
//!
//! ### DME Elevation
//!
//! If unavailable separately, the associated VOR elevation is used.
//!
//! ### Figure of Merit (5.149)
//!
//! - Derived from Navaid Class.
//! - If the Navaid Class is unknown, Figure of Merit = `3`.
//!
//! ### Weather Capability
//!
//! HIWAS and TWEB are coded as:
//!
//! `A`
//!
//! ---
//!
//! # Airways
//!
//! U.S. airways include:
//!
//! - Enroute Airways
//! - Area Navigation Routes
//! - RNAV IFR Terminal Transition Routes
//! - ATS Routes
//!
//! Canadian and other non-U.S. airways are **not included**.
//!
//! ## Route Type (5.7)
//!
//! | Code | Meaning |
//! |------|---------|
//! | O | Conventional |
//! | R | RNAV |
//!
//! The Minimum Altitude field contains:
//!
//! - Conventional MEA for conventional routes.
//! - GNSS MEA for RNAV routes.
//! - Conventional MEA if no GNSS MEA exists.
//!
//! ### Route Level (5.19)
//!
//! | Prefix | Level |
//! |---------|-------|
//! | V, T | L |
//! | J, Q | H |
//! | Others | Blank |
//!
//! ATS Routes do not contain directional restrictions (5.115).
//!
//! ---
//!
//! # Approaches
//!
//! Supported procedures include:
//!
//! - ILS
//! - LOC (Category I only)
//! - VOR
//! - NDB
//! - GPS Overlay
//! - GPS
//! - RNAV (GPS)
//! - RNAV (RNP)
//! - Helicopter GPS/RNAV approaches
//!
//! PI records are only included when the corresponding procedure exists in the CIFP.
//!
//! ## LDA
//!
//! LDA procedures with both:
//!
//! - LDA minima
//! - Glide Slope minima
//!
//! are coded only to the **LDA minima**.
//!
//! ## ARINC 424 Version 19
//!
//! Version 19 applies to:
//!
//! - SID/STAR/Approach Identifier
//! - Route Type
//! - Route Qualifier 1
//! - Level of Service continuation records
//! - Circling procedure altitude coding
//!
//! For RNAV (RNP):
//!
//! - Column 20 = `H`
//! - Column 119 = `F`
//! - Final and missed approach route identifiers begin with `H`.
//!
//! RF legs are coded as fly-by unless followed by an Hx leg.
//!
//! ### Holding
//!
//! Waypoint Description Code `H` is not used for:
//!
//! - Arrivals
//! - SIDs
//! - STAR holding
//!
//! Holding records (EP) are unavailable.
//!
//! ---
//!
//! # Controlled Airspace
//!
//! - Airspace names come from the legal description.
//! - Airspace Center uses the ICAO identifier of the primary airport.
//! - Unit Indicator for altitudes described as **GND** is:
//!
//! `A` (AGL)
//!
//! ---
//!
//! # Special Use Airspace
//!
//! ## Time Code
//!
//! | Value | Meaning |
//! |--------|---------|
//! | C | Continuous |
//! | Blank | Part-time |
//!
//! ### Altitudes
//!
//! - Altitudes are coded only on the first record.
//! - Upper limits are inclusive ("to and including").
//! - Ground-based limits use Unit Indicator `A`.
//!
//! ### Geometry
//!
//! Arc endpoints creating gaps greater than **0.02 NM** may be recalculated to better match published radii.
//!
//! ### Totally Excluded Volumes
//!
//! Lower and upper limits are coded:
//!
//! ```text
//! GND A0000A
//! ```
//!
//! ### Continuation Records
//!
//! Included only for:
//!
//! - Controlling Agencies (5.140)
//!
//! ### Special Air Traffic Rules Areas (SATR)
//!
//! Included as UR records where geometry permits.
//!
//! Restricted Airspace Type:
//!
//! `U`
//!
//! Examples include:
//!
//! - Anchorage SATR
//! - Ketchikan SATR
//! - Grand Canyon SFRA (East and West)
//! - Luke AFB SATR
//! - Washington DC SFRA / FRZ
//! - Niagara Falls SATR
//! - New York SFRA
//! - Valparaiso SATR
//! - Portland (VUO) SFRA
//!
//! National Security Areas are also coded as `U`.
//!
//! ### Grand Canyon SFRA
//!
//! The Grand Canyon SFRA is divided into:
//!
//! - East
//! - West
//!
//! Internal sector boundaries and Flight Free Zones are included.
//!
//! ### Maritime Limits
//!
//! UR airspace based on maritime boundaries is being standardized to NOAA references while preserving legal-description coordinates.
//!
//! ---
//!
//! # Coding Information
//!
//! ## Waypoint Descriptor
//!
//! - Compulsory waypoints are **not indicated** in Waypoint Descriptor 3.
//! - `R` indicates a course change in the final approach.
//! - `S` is **not used**:
//!   - Between FACF and FAF.
//!   - Between FAF and MAP on RNAV (RNP) procedures.
//!
//! ---
//!
//! ## Final Approach Course Fix (FACF)
//!
//! The FACF may also be the Initial Fix (IF), allowing step-down fixes between the FACF and FAF.
//!
//! ---
//!
//! ## Minimum Safe Altitude (MSA)
//!
//! Normally:
//!
//! - MSA Center Fix is coded on the FAF.
//!
//! If the FAF is an RF leg:
//!
//! - Center Fix is coded on the FACF.
//!
//! If no FACF exists:
//!
//! - No center fix is coded.
//!
//! ---
//!
//! ## Vertical Angle
//!
//! The following are coded as:
//!
//! ```text
//! 000
//! ```
//!
//! - Circling procedures
//! - Dive-and-drive procedures
//! - Straight-in procedures where Flight Inspection directs due to obstacles
//!
//! ---
//!
//! ## Missed Approach
//!
//! CA (Course-to-Altitude) legs may be used first.
//!
//! If no mandatory altitude exists:
//!
//! The coded altitude is the lowest of:
//!
//! - Decision Altitude (DA)
//! - Minimum Descent Altitude (MDA)
//! - 400 feet above airport elevation
//!
//! Alternate missed approaches (Route Type `Z`) are **not included**.
//!
//! The missed approach route type matches the final approach route.
//!
//! ---
//!
//! ## Required Navigation Performance (RNP)
//!
//! RNP values follow FAA Order 8260.58C:
//!
//! 1. HF and HM legs receive no RNP values.
//! 2. Amendments match FAA Form 8260-3.
//! 3. P-NOTAM coding changes use FAA Form 8260-3 values.
//! 4. P-NOTAMs without coding changes are updated as workload permits.
//! 5. Existing procedures are updated gradually, with preference given to updating all procedures at an airport together.
//!
//! ---
//!
//! ## Route Qualifier 2
//!
//! Route Qualifier 2:
//!
//! ```text
//! H
//! ```
//!
//! is used for helicopter approaches using helipads.
//!
//! ---
//!
//! ## File Record Number (5.31)
//!
//! - Unique per record.
//! - Not consecutive across the dataset.
//! - May contain alphabetic characters or blanks.
//!
//! ---
//!
//! ## Cycle Date (5.32)
//!
//! Cycle dates are updated to the latest cycle for:
//!
//! - Newly added records
//! - Modified records
//!
//!
//!

pub mod definitions;
pub mod entities;
pub mod records;
