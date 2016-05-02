# A Web Server in Rust

 The focus of this project was to leverage the features of the relatively new Rust programming language in order to build a scalable web server. The implementation began as a very basic and naive web server. However, the architecture gradually grew in size as a thread pool was integrated in order to enable parallelization of tasks, priority scheduling was introduced to prevent large file requests from clogging the web server traffic, and server side caching was added to reduce the number of necessary I/O operations. Logging the incoming requests and outgoing response statuses was also implemented across the threads in the thread pool. The ApacheBench tool was used to measure the performance at the various stages of the web server’s progression. These benchmarks were compared with those of the Apache web server in order to determine whether a Rust web server is capable of matching the scale an existing web server that is widely used today. This initial evaluation suggests that the Rust web server design without the prioritization is comparable and perhaps slightly better in performance than the Apache web server for smaller files. The priority scheduling effectively serves files in order of file size, however, continued work is required to improve the overall performance. Additional work also includes adding other typical features found in existing web servers.

## Web server Architecture
 ![alt text](https://github.com/Arulselvanmadhavan/rust-engine/blob/master/web_server/project_images/final_architecture.png "Web Server Architecture")

## Build & Installation:
1. Go to https://github.com/brson/multirust to install multirust.
   Ideally, the following command should work:
   ```
   curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh
   ```
   multirust essentially allows you to have several versions of Rust installed at
   once. In order to use the concurrent hash map library, we needed to use a nightly
   build of Rust.
2. Once multirust is installed, type the following commands in the terminal:
   ```
   multirust update nightly
   ```
   ```
   multirust default nightly
   ```
3. To build the Rust web server, run:
   ```rust
   cargo build
   ```
4. To run the Rust web server, run:
   cargo run [# of scheduler threads] [# of worker threads] [# of logger threads] [# cache threads]

   For example, to run with 2 scheduler threads, 5 worker threads, 1 logger thread, and 1 cache thread, you would run:
   ```rust
   cargo run 2 5 1 1
   ```
5. The server will be listening on localhost port 8080, so for example you can
   open the Chrome browser and go to localhost:8080/foo.html, and you should
   see a Hello, World! web page displayed.
   Note that it worked in the FireFox browser for OSX, but for some reason it does
   not seem to display correctly in the FireFox browser in an Ubuntu VM. Chrome
   seems to work in the Ubuntu VM, however. All browsers seemed to work on OSX.

Other notes:
The source code is in the /src directory. The /examplefiles directory
contains the files of various sizes used for benchmarking. The Cargo.toml
file contains configurations for the cargo package manager, including
dependencies of the project. The /target directory is the output directory
as usual.
### Team Members
Jimmy Ly and Arulselvan Madhavan
