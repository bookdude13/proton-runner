# proton-runner
Show runner for Proton

# Dependencies

## SFML/CSFML
proton-runner depends on rust-sfml. To setup dependencies, Unix users should install libcsfml-dev and libsfml-dev using aptitude, homebrew, or any other package manager you are using. Windows users should create the following environment variables:  

|Name|Value|
|---|---|
|CSFML_HOME|full/path/to/proton-runner/libs/CSFML|
|SFML_HOME|full/path/to/proton-runner/libs/SFML|

You may also need to add proton-runner/libs/CSFML/bin and proton-runner/libs/SFML/bin to your PATH variable in order to run proton-runner.

CSFML and SFML licenses are found in their corresponding subdirectories. Copies can also be found in licenses/. The only changes to the source have been deleting the include/ and doc/ folders for CSFML and the cmake, doc, examples, and include folders for SFML (done for space). Credit to Laurent Gomila for both libraries. Using CSFML version 2.4 and SFML version 2.4.2

## SSL
proton-runner also depends on rust-openssl, which prompts the following notice:  
"This product includes software developed by the OpenSSL Project for use in the OpenSSL Toolkit (http://www.openssl.org/)". The full license(s) can be found in licenses/rust-openssl_licenses

To install dependencies on Unix systems, install either libssl-dev, openssl-devel, or openssl (try in that order; depends on the OS). You will also need pkg-config. Refer to [rust-openssl](https://github.com/sfackler/rust-openssl) if you have any issues.
