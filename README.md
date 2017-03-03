# proton-runner
Show runner for Proton

# SFML/CSFML (and deps)
proton-runner depends on rust-sfml. To get the external dependencies for rust-sfml in the right place, proton-runner sets up CSFML and SFML libraries when on Windows machines. Unix users should install libcsfml-dev and libsfml-dev using aptitude, homebrew, or any other package manager you are using.  
CSFML and SFML licenses are found in their corresponding subdirectories. Copies can also be found in licenses/. The only changes to the source have been deleting the include/ and doc/ folders for CSFML and the cmake, doc, examples, and include folders for SFML (done for space). Credit to Laurent Gomila for both libraries.

# SSL
proton-runner also depends on rust-openssl, which prompts the following notice:  
"This product includes software developed by the OpenSSL Project for use in the OpenSSL Toolkit (http://www.openssl.org/)". The full license(s) can be found in licenses/rust-openssl_licenses

To install dependencies on Unix systems, install either libssl-dev, openssl-devel, or openssl (try in that order; depends on the OS). You will also need pkg-config. Refer to [rust-openssl](https://github.com/sfackler/rust-openssl) if you have any issues.
