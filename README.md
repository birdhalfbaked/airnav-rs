# Lib-airnav

This project helps process air navigation data in a structured way. It targets **ARINC 424** today and is intended to grow toward **AIXM** sources as well.

After reviewing both data, a format that will make sense across both will emerge and offer a standard representation that can be used with both
sources.

## Goals

- Bridge parsing/verification/export of the two standards present for FMS data:
    - AIXM
    - ARINC + extra XML
- Enable a standardized representation that allows users to quickly translate data to their needs within Rust applications
- Fix some of the issues with representation of data using a higher-level layer that can feed data from multiple sources

## ARINC 424

Parsing and definitions follow **ARINC 424-23** (Specification 424, Change 23).

- Raw field definitions with human verification through field **5.148**
- Record parsing soon after all the human verification of the raw fields is done. No shortcuts

## AIXM

Coming soon, though should be easier as the schema is well defined and fits within XML parsing semantics nicely already