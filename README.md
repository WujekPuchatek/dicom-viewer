# DICOM Viewer

This application is written in Rust and provides functionality for parsing and viewing DICOM files in 3D or as montage of 2D images.

## Features

- DICOM File Parsing: The library can parse DICOM files and extract the metadata and data elements. It uses a `DicomFileParser` struct for this purpose. The parser can read tags, value representations, and value lengths from the DICOM file.

- DICOM File Validation: The library includes a `Validator` struct that checks if a file is a valid DICOM file.

- Tag Reading: The library can read specific tags from the DICOM file. The tags to read can be specified when creating a `DicomFileParser` instance.

## Usage

To use the library, create an instance of `DicomFileParser` with the path to the DICOM file. Then, call the `parse` method on the parser instance.
