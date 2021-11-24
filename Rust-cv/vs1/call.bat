REM Minimal params to run the vslam-sandbox. The ARGS are the input images, separated by spaces
REM Must set the env variable RUST_LOG=trace  or debug/info
cmd /v /c "set RUST_LOG=trace&& F:\rustcv\cv\target\debug\vslam-sandbox.exe --output P:\
REM F:\rustcv\cv\vslam-sandbox>cmd /v /c "set RUST_LOG=trace&& F:\rustcv\cv\target\debug\vslam-sandbox.exe --output P:\ P:\image1.jpg P:\image2.jpg P:\image3.jpg

REM fn main() vslam tutorial four?
REM vslam.cvr, vslam-settings.json, 0
REM 2021-11-23T23:39:06.832Z INFO  vslam_sandbox > trying to load existing reconstruction data
REM 2021-11-23T23:39:06.833Z INFO  vslam_sandbox > used empty reconstruction
REM 2021-11-23T23:39:06.835Z INFO  vslam_sandbox > used default settings
REM 2021-11-23T23:39:06.835Z INFO  vslam_sandbox > reconstruction not modified, so not saving reconstruction data


