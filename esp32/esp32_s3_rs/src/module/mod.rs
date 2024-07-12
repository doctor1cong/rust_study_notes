use app_thread::app_thread_start;
use esp_idf_hal::task::*;

pub mod app_thread;

pub fn app1 ()
{
    app_thread_start();
}