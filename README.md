# Licemage — Image Licence Scanner.

Image licence scanner is another addition to SBOM process.

It is intended to be used to extract all licences of the installed packages.
The goal is to provide a distribution-agnostic tool that can do the following
operations:

- Extract and display licences, those are claimed in the package metadata
- Verify if claimed licences are corresponding to the actual licences in the sources, helping finding incorrect metadata for further fixing
- Reconcile SPDX data

# Limitations

This tool requires a distribution with a Package Manager.

## Current Focus

Currently tool is focusing on working with Debian distributions, however other
package managers can be easily added.
