/*!
 *  # Logitech R400 Custom driver #
 *  <center>
 *      <img src="../../../../assets/R400.jpeg" height="200" />
 *  </center>
 *
 *  This module mainly exposes:
 *   - the [device::get] function to discover a Logitech USB Receiver device,   
 *   - the [Driver] structure, allowing to control the Logitech R400 device.
 *
 *  The driver allows for three different modes of [behaviours] (grouped in the [DriverMode](behaviours::DriverMode) enum):
 *   - [MouseMode](behaviours::DriverMode::MouseMode): controls the mouse using left/right (and up/down by holding the bottom-right button),
 *   - [MorseMode](behaviours::DriverMode::MorseMode): translates the presses on the bottom-right button as morse code and inputs the corresponding characters,
 *   - [PlayerMode](behaviours::DriverMode::PlayerMode): basic controls for media player (rewind/forward, play/pause...).
 *
 *   Switching between modes is done by holding both left and right buttons.
 */

pub mod device;
pub mod keys;

mod behaviours;
mod driver;
mod state;
mod utils;

pub use driver::Driver;
