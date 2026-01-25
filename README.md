# Psyche Raw Image Utilities

Psyche Raw Utils (PRU) is a set of utilities for the retrieval, calibration, and manipulation of publically available raw Psyche mission imagery. It is not meant or intended to work with or produce full comprehensive science products (that is left to the NASA Planetary Data System and traditional image processing toolsets), instead provide tools for the enthusiast and "Citizen Scientist" communities to streamline, standardize, and teach the operations generally used for flight mission image processing. 

## Image Source:
 Images are to be sourced from the official NASA-supported raw image collection: https://solarsystem.nasa.gov/psyche-raw-images/

## Quick Start

## Contributing
Feedback, issues, and contributions are always welcomed. Should enough interest arise in contributing development efforts, I will write up a contribution guide. 

## Citing Psyche Raw Utils
Citing PRU is not required, but if the software has significantly contributed to your research or if you'd like to acknowledge the project in your works, I would be grateful if you did so.  

## Building from source
A working Rust (https://www.rust-lang.org/) installation is required for building. PRU targets the 2021 edition, stable branch. 

### Clone from git
```bash
git clone git@github.com:kmgill/psyche_raw_utils.git
cd psyche_raw_utils/
```
### Install via cargo
This is the easiest installation method for *nix-based systems, though it does require a working installation of the Rust toolchain. While the software does build and run natively on Windows, it is recommended to be used within a Ubuntu container on the Windows Subsystem for Linux.

```
...
```

## References

Bell, J.F., Ravine, M.A., Caplinger, M.A. et al. The Psyche Multispectral Imager Investigation: Characterizing the Geology, Topography, and Multispectral Properties of a Metal-Rich World. Space Sci Rev 221, 47 (2025). https://doi.org/10.1007/s11214-025-01169-3 https://doi.org/10.1007/s11214-025-01169-3

Jaumann, R., Bell, J.F., Polanskey, C.A. et al. The Psyche Topography and Geomorphology Investigation. Space Sci Rev 218, 7 (2022). https://doi.org/10.1007/s11214-022-00874-7

Telea, Alexandru. (2004). An Image Inpainting Technique Based on the Fast Marching Method. Journal of Graphics Tools. 9. 10.1080/10867651.2004.10487596. 
https://www.researchgate.net/publication/238183352_An_Image_Inpainting_Technique_Based_on_the_Fast_Marching_Method

