
use opencv::{
    core,
    highgui,
    imgproc,
    prelude::*,
    videoio,
    Result,
};

const LIVE_ORIGINAL: &str = "Live - Original";
const LIVE_FILTERED: &str = "Live - Grayscale";
const STILL_ORIGINAL: &str = "Captured - Original";
const STILL_FILTERED: &str = "Captured - Grayscale";

// Converts a BGR color image into grayscale.
fn apply_grayscale(frame: &core::Mat) -> Result<core::Mat> {
    let mut gray = core::Mat::default();

    imgproc::cvt_color(
        frame,
        &mut gray,
        imgproc::COLOR_BGR2GRAY,
        0,
    )?;

    Ok(gray)
}

// Creates the two windows for the live camera.
fn create_windows() -> Result<()> {
    highgui::named_window(
        LIVE_ORIGINAL,
        highgui::WINDOW_AUTOSIZE,
    )?;

    highgui::named_window(
        LIVE_FILTERED,
        highgui::WINDOW_AUTOSIZE,
    )?;

    Ok(())
}

// Captures an independent copy of the latest frame.
// The snapshot does not change when new frames arrive.
fn capture_still(frame: &core::Mat) -> Result<()> {
    let snapshot = frame.try_clone()?;
    let filtered_snapshot = apply_grayscale(&snapshot)?;

    highgui::named_window(
        STILL_ORIGINAL,
        highgui::WINDOW_AUTOSIZE,
    )?;

    highgui::named_window(
        STILL_FILTERED,
        highgui::WINDOW_AUTOSIZE,
    )?;

    highgui::imshow(STILL_ORIGINAL, &snapshot)?;
    highgui::imshow(STILL_FILTERED, &filtered_snapshot)?;

    println!("Still image captured!");
    Ok(())
}

// Continuously receives, processes, and displays frames.
fn camera_loop(camera: &mut videoio::VideoCapture) -> Result<()> {
    let mut failed_frames = 0;

    loop {
        let mut frame = core::Mat::default();

        let successful = camera.read(&mut frame)?;

        // Some webcams occasionally return an empty frame.
        if !successful || frame.empty() {
            failed_frames += 1;
            eprintln!("Warning: failed to read camera frame.");

            if failed_frames >= 30 {
                return Err(opencv::Error::new(
                    core::StsError,
                    "Camera stopped providing frames",
                ));
            }

            // Keep GUI events responsive while retrying.
            let key = highgui::wait_key(30)?;
            if key == 113 || key == 27 {
                break;
            }
            continue;
        }

        failed_frames = 0;

        // Filter the live camera frame.
        let filtered = apply_grayscale(&frame)?;

        // Display original and filtered live video.
        highgui::imshow(LIVE_ORIGINAL, &frame)?;
        highgui::imshow(LIVE_FILTERED, &filtered)?;

        // Check keyboard input approximately every 30 ms.
        let key = highgui::wait_key(30)?;

        match key {
            99 | 67 => {
                // 'c' or 'C' captures the current frame.
                capture_still(&frame)?;
            }

            113 | 81 | 27 => {
                // 'q', 'Q', or Esc exits the application.
                println!("Closing webcam application...");
                break;
            }

            _ => {}
        }

        // Closing a live window also stops the program.
        if highgui::get_window_property(
            LIVE_ORIGINAL,
            highgui::WND_PROP_VISIBLE,
        )? < 1.0 {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    println!("Rust Webcam Application");
    println!("C = Capture image");
    println!("Q or Esc = Quit");

    // 0 usually corresponds to /dev/video0.
    // CAP_V4L2 requests the Linux Video4Linux2 backend.
    let mut camera = videoio::VideoCapture::new(
        0,
        videoio::CAP_V4L2,
    )?;

    // Check whether the camera connection succeeded.
    if !camera.is_opened()? {
        eprintln!("ERROR: Could not open webcam.");
        eprintln!("Check /dev/video0 and VirtualBox passthrough.");

        return Err(opencv::Error::new(
            core::StsError,
            "Webcam connection failed",
        ));
    }

    println!("Camera opened successfully.");

    // Request a moderate resolution for performance.
    camera.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0)?;
    camera.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0)?;

    // Execute the GUI, retaining the result for cleanup.
    let result = (|| -> Result<()> {
        create_windows()?;
        camera_loop(&mut camera)
    })();

    // Release the device even if the loop reported an error.
    let release_result = camera.release();
    let windows_result = highgui::destroy_all_windows();

    // Preserve the main application error when possible.
    result?;
    release_result?;
    windows_result?;

    println!("Camera resources released.");
    Ok(())
}
